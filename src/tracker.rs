use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
    sync::Mutex,
};

use crate::{DogstatsdClient, TagsProvider};

/// See https://www.datadoghq.com/pricing/ and https://docs.datadoghq.com/account_management/billing/custom_metrics/,
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

    /// Track the given tags, and take the user defined action if the cardinality threshold is reached.
    ///
    /// Returns the tags as-is, so that it can be used in a `DogstatsdClient` call.
    pub(crate) fn wrap_and_track<S, T>(&self, dd: &impl DogstatsdClient, metric: &str, tags: T) -> T
    where
        S: AsRef<str>,
        T: TagsProvider<S>,
    {
        // This function performs some magic that reduces binary size and number of allocations.
        // It's not obvious, so here is a description of what it does:
        // 1. It takes a `impl TagsProvider<impl AsRef<str>>` and turns it into a `impl Iterator<Item = &str>`
        // 2. This iterator is then turned into a `RewindableTagsIter` which is a wrapper around the iterator
        //    that allows us to rewind it to the beginning at our pleasure.
        // 3. We use `dyn` to erase the type of the iterator, reducing binary size as it's better monomorphized.

        {
            // Get a slice of `&[impl AsRef<str>]`
            let tags = tags.as_ref();
            // Remember its length
            let len = tags.len();
            // Create an iterator that will infinitely cycle through the tags, returning `&str`s
            let mut tags = tags.iter().map(|s| s.as_ref()).cycle();
            // Create a stack allocated dyn iterator to put in the `RewindableTagsIter`
            let tags: &mut dyn Iterator<Item = &str> = &mut tags;
            // Construct a `RewindableTagsIter` that will allow us to rewind the iterator when needed
            let tags = RewindableTagsIter { cursor: 0, tags, len };
            // Now we can call `track`!
            self.track(dd, metric, tags);
        }

        // Return the tags as-is for convenience
        tags
    }

    fn track(&self, dd: &impl DogstatsdClient, metric: &str, mut tags: RewindableTagsIter) {
        let mut lock = match self.state.as_ref() {
            Some(state) => state.lock().unwrap(),
            None => return,
        };
        let state = std::mem::replace(&mut *lock, TrackerState::Done);
        match state {
            TrackerState::Running {
                mut seen,
                mut cardinality_count,
                actions,
            } => {
                let cardinality_grown = Self::update(&mut seen, metric, tags.rewind());
                if cardinality_grown {
                    cardinality_count += 1;
                    if cardinality_count >= self.cardinality_threshold {
                        drop(lock);
                        Self::do_actions(dd, seen, actions, metric, tags.rewind());
                        return;
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
    }

    fn update(seen: &mut BTreeMap<String, Vec<BTreeSet<String>>>, metric: &str, tags: &mut RewindableTagsIter) -> bool {
        let seen_tag_set = match seen.get_mut(metric) {
            Some(seen_tag_set) => seen_tag_set,
            None => {
                seen.insert(
                    metric.to_string(),
                    vec![BTreeSet::from_iter(tags.rewind().map(|tag| tag.to_string()))],
                );
                return true;
            }
        };
        // Is this set of tags new for this metric?
        let set_is_novel = seen_tag_set
            .iter()
            .all(|tag_set| tag_set.len() != tags.len() || tags.rewind().any(|tag| !tag_set.contains(tag)));
        if set_is_novel {
            seen_tag_set.push(BTreeSet::from_iter(tags.map(|tag| tag.to_string())));
        };
        set_is_novel
    }

    fn do_actions(
        dd: &impl DogstatsdClient,
        seen: BTreeMap<String, Vec<BTreeSet<String>>>,
        actions: Vec<ThresholdAction>,
        metric: &str,
        tags: &mut RewindableTagsIter,
    ) {
        let event_tags = seen
            .iter()
            .map(|(metric, tags)| format!("{}:{}", metric, tags.len()))
            .collect::<Vec<_>>();

        let tags = tags.rewind().collect::<Vec<&str>>();

        for action in actions {
            match action {
                ThresholdAction::Event { title, text } => dd.event(&title, &text, &event_tags),
                ThresholdAction::Custom(mut action) => {
                    action(metric, &tags);
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

    /// Take some custom action. The function will be passed the metric name and tags
    Custom(ThresholdCustomAction),
}

type ThresholdCustomAction = Box<dyn FnMut(&str, &[&str]) + Send + Sync>;

/// The configuration for the tag tracker. By default, the tag tracking is not enabled.
/// To enable it, set the `count_threshold` to a non-zero value, and add at least one event
/// or custom action
/// Example usage:
/// ```rust
/// use prima_datadog::{
///     configuration::{Country, PrimaConfiguration},
///     Datadog, TagTrackerConfiguration,
/// };
/// let tracker_config = TagTrackerConfiguration::new()
///     .with_threshold(21)
///     .with_custom_action(|_, _| {});
/// let configuration = PrimaConfiguration::new(
///     "0.0.0.0:1234",
///     "0.0.0.0:0",
///     "prima_datadog_benchmarks",
///     "dev".parse().unwrap(),
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
    /// Any number of actions may be added.
    ///
    /// # Example
    ///
    /// ```rust
    /// prima_datadog::TagTrackerConfiguration::new().with_custom_action(|metric, tags| {
    ///     println!("Exceeded custom metric threshold for metric {} with tags {:?}", metric, tags);
    /// });
    /// ```
    pub fn with_custom_action(mut self, custom_action: impl FnMut(&str, &[&str]) + Send + Sync + 'static) -> Self {
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
    /// See https://docs.datadoghq.com/developers/metrics/custom_metrics/ for
    /// more information.
    pub fn with_threshold(mut self, count_threshold: usize) -> Self {
        self.count_threshold = count_threshold;
        self
    }

    pub(crate) fn build(self) -> Tracker {
        Tracker::new(self.count_threshold, self.actions)
    }
}

/// An iterator over tags, which can be rewound to the beginning.
struct RewindableTagsIter<'a> {
    tags: &'a mut dyn Iterator<Item = &'a str>,
    cursor: usize,
    len: usize,
}
impl RewindableTagsIter<'_> {
    fn rewind(&mut self) -> &mut Self {
        for _ in 0..(self.len - self.cursor) {
            let _next = self.tags.next();
            debug_assert!(_next.is_some(), "The `tags` in `RewindableTagsIter` should never stop returning items, use `.cycle()` to make it infinite");
        }
        self.cursor = 0;
        self
    }
}
impl<'a> Iterator for RewindableTagsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor < self.len {
            self.cursor += 1;
            self.tags.next()
        } else {
            None
        }
    }
}
impl ExactSizeIterator for RewindableTagsIter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.len
    }
}
