use std::{
    collections::{BTreeMap, BTreeSet},
    iter::FromIterator,
    sync::{Mutex, MutexGuard},
};

use crate::Datadog;

/// See https://www.datadoghq.com/pricing/ and https://docs.datadoghq.com/account_management/billing/custom_metrics/,
///
/// 100 seems like a reasonable place to start warning for now
pub const DEFAULT_TAG_THRESHOLD: usize = 100;

pub(crate) struct TrackerState {
    /// Threshold at which to take the user defined action, and stop tracking
    cardinality_threshold: usize,

    /// The number of unique sets of tags seen for each metric
    cardinality_count: usize,

    /// For each metric, store cardinality
    seen: BTreeMap<String, BTreeSet<BTreeSet<String>>>,

    /// Actions to take when the threshold is exceeded
    actions: Vec<ThresholdAction>,
}

impl TrackerState {
    fn update(mut state: MutexGuard<'_, Self>, dd: &Datadog, metric: &str, tags: &[String]) {
        debug_assert!(
            !tags.is_empty(),
            "update called with empty tags - should have been caught by Tracker::track"
        );

        let seen_tag_sets = match state.seen.get_mut(metric) {
            Some(seen_tag_sets) => seen_tag_sets,
            None => {
                // This isn't very efficient, but because this code path is only taken once per metric, it's not a big deal.
                // Doing this is far better than having to allocate the metric to a String for every call.
                state.seen.insert(metric.to_string(), Default::default());
                state.seen.get_mut(metric).unwrap()
            }
        };

        // Is this set of tags new for this metric?
        let set_is_novel = seen_tag_sets
            .iter()
            .all(|tag_set| tag_set.len() != tags.len() || tags.iter().any(|tag| !tag_set.contains(tag)));

        if set_is_novel {
            seen_tag_sets.insert(BTreeSet::from_iter(tags.iter().cloned()));
            state.cardinality_count += 1;

            // Check if we've exceeded the threshold
            if state.cardinality_count >= state.cardinality_threshold {
                Self::do_actions(state, dd, metric, tags);
            }
        }
    }

    fn do_actions(mut lock: MutexGuard<'_, Self>, dd: &Datadog, metric: &str, tags: &[String]) {
        let actions = core::mem::take(&mut lock.actions);
        if actions.is_empty() {
            debug_assert!(
                false,
                "do_actions called with no actions - should have been caught by Tracker::state or Tracker::new"
            );
            return;
        }

        let event_tags = lock.generate_event_tags();

        // We can deallocate the memory for the seen tags now, since we're going to stop tracking
        lock.seen = Default::default();

        // We REALLY don't want to hold the lock while we run the actions, since a user
        // could e.g. call incr from within a custom action, and deadlock the whole app
        drop(lock);

        for action in actions {
            match action {
                ThresholdAction::Event { title, text } => dd.do_event(title, text, event_tags.clone()),
                ThresholdAction::Custom(mut action) => {
                    action(metric, tags);
                }
            }
        }
    }

    fn generate_event_tags(&self) -> Vec<String> {
        self.seen
            .iter()
            .map(|(metric, unique_tag_sets)| format!("{}:{}", metric, unique_tag_sets.len()))
            .collect()
    }
}

pub(crate) struct Tracker(Option<Mutex<TrackerState>>);

impl Tracker {
    fn new(cardinality_threshold: usize, actions: Vec<ThresholdAction>) -> Self {
        Tracker(if !actions.is_empty() {
            Some(Mutex::new(TrackerState {
                cardinality_threshold,
                actions,

                seen: Default::default(),
                cardinality_count: 0,
            }))
        } else {
            None
        })
    }

    fn state(&self) -> Option<MutexGuard<'_, TrackerState>> {
        let lock = self.0.as_ref()?.lock().unwrap();
        if !lock.actions.is_empty() {
            Some(lock)
        } else {
            None
        }
    }

    pub(crate) fn track(&self, dd: &Datadog, metric: &str, tags: &[String]) {
        if tags.is_empty() {
            // If the tags are empty we don't need to track them
            return;
        }
        if let Some(state) = self.state() {
            TrackerState::update(state, dd, metric, tags)
        }
    }
}

/// Actions that define what the tracker will do when the custom metric threshold is passed.
// A user may define any number of these, and by default none are taken.
enum ThresholdAction {
    /// Emit an event. The count of unique tag sets, per metric, is provided as the tags
    /// for the event, i.e. for a given metric `metric`, there will be a tag `metric:count`,
    /// where count is the number of unique tag sets seen for that metric.
    Event { title: String, text: String },

    /// Take some custom action. The function will be passed the metric name and tags
    Custom(ThresholdCustomAction),
}

type ThresholdCustomAction = Box<dyn for<'a> FnMut(&'a str, &'a [String]) + Send + Sync>;

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

    pub fn with_event(mut self, title: String, text: String) -> Self {
        self.actions.push(ThresholdAction::Event { title, text });
        self
    }

    /// Add a custom action to execute when the custom metric threshold is passed.
    ///
    /// # Example
    ///
    /// ```rust
    /// prima_datadog::TagTrackerConfiguration::new().with_custom(|metric: &str, tags: &[String]| {
    ///     println!("Exceeded custom metric threshold for metric {} with tags {:?}", metric, tags);
    /// });
    /// ```
    pub fn with_custom(mut self, custom_action: impl FnMut(&str, &[String]) + Send + Sync + 'static) -> Self {
        self.actions
            .push(ThresholdAction::Custom(Box::new(custom_action) as Box<_>));
        self
    }

    pub fn with_threshold(mut self, count_threshold: usize) -> Self {
        self.count_threshold = count_threshold;
        self
    }

    pub(crate) fn build(self) -> Tracker {
        Tracker::new(self.count_threshold, self.actions)
    }
}
