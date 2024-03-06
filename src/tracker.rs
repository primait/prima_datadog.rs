use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    iter::FromIterator,
    sync::Mutex,
};

use crate::{DogstatsdClient, TagsProvider};

/// See <https://www.datadoghq.com/pricing/> and <https://docs.datadoghq.com/account_management/billing/custom_metrics/>,
///
/// 100 seems like a reasonable place to start warning for now
pub const DEFAULT_TAG_THRESHOLD: usize = 100;

enum TrackerState {
    Running {
        seen: BTreeMap<String, Vec<BTreeSet<String>>>,
        cardinality_count: usize,
        actions: Vec<ThresholdAction>,
    },
    Done,
}

pub(crate) struct Tracker {
    /// Threshold at which to take the user defined action, and stop tracking
    cardinality_threshold: usize,

    /// Our internal state
    ///
    /// This will be `None` if the user does not want to track cardinality.
    state: Option<Mutex<TrackerState>>,
}

impl Tracker {
    fn new(cardinality_threshold: usize, actions: Vec<ThresholdAction>) -> Self {
        Tracker {
            cardinality_threshold,
            state: if !actions.is_empty() && cardinality_threshold != 0 {
                Some(Mutex::new(TrackerState::Running {
                    actions,
                    seen: Default::default(),
                    cardinality_count: 0,
                }))
            } else {
                None
            },
        }
    }

    pub(crate) fn track<S, T>(&self, dd: &impl DogstatsdClient, metric: &str, tags: T) -> T
    where
        S: AsRef<str>,
        T: TagsProvider<S>,
    {
        let mut lock = match self.state.as_ref() {
            Some(state) => state.lock().unwrap(),
            None => return tags,
        };
        let tag_slice = tags.as_ref();
        let state = std::mem::replace(&mut *lock, TrackerState::Done);
        match state {
            TrackerState::Running {
                mut seen,
                mut cardinality_count,
                actions,
            } => {
                let cardinality_grown = Self::update(&mut seen, metric, tag_slice);
                if cardinality_grown {
                    cardinality_count += 1;
                    if cardinality_count >= self.cardinality_threshold {
                        drop(lock);
                        Self::do_actions(dd, seen, actions, metric, tag_slice);
                        return tags;
                    }
                }
                // Rebuild the running state if we didn't run the actions
                *lock = TrackerState::Running {
                    seen,
                    cardinality_count,
                    actions,
                };
            }
            TrackerState::Done => {}
        };
        tags
    }

    fn update(seen: &mut BTreeMap<String, Vec<BTreeSet<String>>>, metric: &str, tags: &[impl AsRef<str>]) -> bool {
        let seen_tag_set = match seen.get_mut(metric) {
            Some(seen_tag_set) => seen_tag_set,
            None => {
                seen.insert(
                    metric.to_string(),
                    vec![BTreeSet::from_iter(tags.iter().map(|tag| tag.as_ref().to_string()))],
                );
                return true;
            }
        };
        // Is this set of tags new for this metric?
        let set_is_novel = seen_tag_set
            .iter()
            .all(|tag_set| tag_set.len() != tags.len() || tags.iter().any(|tag| !tag_set.contains(tag.as_ref())));
        if set_is_novel {
            seen_tag_set.push(BTreeSet::from_iter(tags.iter().map(|tag| tag.as_ref().to_string())));
        };
        set_is_novel
    }

    fn do_actions(
        dd: &impl DogstatsdClient,
        seen: BTreeMap<String, Vec<BTreeSet<String>>>,
        actions: Vec<ThresholdAction>,
        metric: &str,
        tags: &[impl AsRef<str>],
    ) {
        let event_tags = seen
            .iter()
            .map(|(metric, tags)| format!("{}:{}", metric, tags.len()))
            .collect::<Vec<_>>();

        let tags = tags.iter().map(|t| t.as_ref()).collect::<Vec<&str>>();
        // Map from a BTreeMap<String, Vec<BTreeSet<String>>> to a HashMap<String, Vec<HashSet<String>>>
        let seen: HashMap<String, Vec<HashSet<String>>> = seen
            .into_iter()
            .map(|(metric, sets)| (metric, sets.into_iter().map(|set| set.into_iter().collect()).collect()))
            .collect();
        for action in actions {
            match action {
                ThresholdAction::Event { title, text } => dd.event(&title, &text, &event_tags),
                ThresholdAction::Custom(mut action) => {
                    action(metric, &tags, &seen);
                }
            }
        }
    }
}

/// Actions that define what the tracker will do when the custom metric threshold is passed.
/// A user may define any number of these, and by default none are taken.
enum ThresholdAction {
    /// Emit an event. The count of unique tag sets, per metric, is provided as the tags
    Event { title: String, text: String },

    /// Take some custom action.
    Custom(ThresholdCustomAction),
}

type ThresholdCustomAction = Box<dyn FnMut(&str, &[&str], &HashMap<String, Vec<HashSet<String>>>) + Send + Sync>;

/// The configuration for the tag tracker. By default, the tag tracking is not enabled.
/// To enable it, set the `count_threshold` to a non-zero value, and add at least one event
/// or custom action
/// Example usage:
/// ```rust
/// use prima_datadog::{
///     configuration::{Country, Configuration},
///     Datadog, TagTrackerConfiguration,
/// };
/// let tracker_config = TagTrackerConfiguration::new()
///     .with_threshold(21)
///     .with_custom_action(|_, _, _| {});
/// let configuration = Configuration::new(
///     "0.0.0.0:1234",
///     "prima_datadog_benchmarks",
/// ).with_country(Country::It).with_tracker_configuration(tracker_config);
/// Datadog::init(configuration).unwrap();
/// ```
pub struct TagTrackerConfiguration {
    count_threshold: usize,
    actions: Vec<ThresholdAction>,
}

impl Default for TagTrackerConfiguration {
    fn default() -> Self {
        Self {
            count_threshold: DEFAULT_TAG_THRESHOLD,
            actions: Vec::new(),
        }
    }
}

impl TagTrackerConfiguration {
    pub fn new() -> Self {
        Self {
            count_threshold: DEFAULT_TAG_THRESHOLD,
            actions: Vec::new(),
        }
    }

    /// Configure the tracker to emit an event when the threshold is reached.
    /// The count of unique tag sets, per metric, is provided as the tags
    /// for the event, i.e. for a given metric `metric`, there will be a tag `metric:count`,
    /// where count is the number of unique tag sets seen for that metric.
    /// Any number of events may be configured, and all will be emitted when the
    /// threshold is reached.
    pub fn with_event(mut self, title: String, text: String) -> Self {
        self.actions.push(ThresholdAction::Event { title, text });
        self
    }

    /// Add a custom action to execute when the custom metric threshold is reached.
    /// These actions are run exactly once, at the point the threshold is reached.
    /// Any number of actions may be added. The function will be passed the metric name,
    /// the tags of the metric causing the crossing of the threshold, and a
    /// HashMap containing all the unique tag sets seen for each metric.
    ///
    /// # Example
    ///
    /// ```rust
    /// prima_datadog::TagTrackerConfiguration::new().with_custom_action(|metric, tags, _| {
    ///     println!("Exceeded custom metric threshold for metric {} with tags {:?}", metric, tags);
    /// });
    /// ```
    pub fn with_custom_action(
        mut self,
        custom_action: impl FnMut(&str, &[&str], &HashMap<String, Vec<HashSet<String>>>) + Send + Sync + 'static,
    ) -> Self {
        self.actions
            .push(ThresholdAction::Custom(Box::new(custom_action) as Box<_>));
        self
    }

    /// Set the threshold for the maximum number of custom metrics
    /// This defaults to ```DEFAULT_TAG_THRESHOLD```
    ///
    /// The threshold is the maximum number of "custom metrics" which can
    /// be generated before the tracker actions are run.
    ///
    /// A "custom metric" is defined as the unique combination of metric name
    /// and unique tag set, e.g., a metric, "test", with the following tag sets:
    /// - [a, b, c]
    /// - [a, b, d]
    /// - [a, c, d]
    /// is counted as 3 "custom metrics", as there are 3 unique tag sets here.
    ///
    /// See <https://docs.datadoghq.com/developers/metrics/custom_metrics/> for
    /// more information.
    pub fn with_threshold(mut self, count_threshold: usize) -> Self {
        self.count_threshold = count_threshold;
        self
    }

    pub(crate) fn build(self) -> Tracker {
        Tracker::new(self.count_threshold, self.actions)
    }
}
