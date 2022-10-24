use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

use crate::Datadog;

/// See https://www.datadoghq.com/pricing/ and https://docs.datadoghq.com/account_management/billing/custom_metrics/,
/// 100 seems like a reasonable place to start warning for now
pub const DEFAULT_TAG_THRESHOLD: usize = 100;

pub(crate) struct Tracker {
    // Threshold at which to take the user defined action, and stop tracking
    count_threshold: usize,
    // Actions to take when the threshold is exceeded
    actions: Mutex<Vec<ThresholdAction>>,
    // State
    state: Mutex<TrackerState>,
}

enum TrackerState {
    Tracking(TrackingState), // Currently tracking metric counts
    Done,
}

#[derive(Debug)]
struct TrackingState {
    // For each metric, store the list of sets of unique tag key:value pairs seen. Yes, this hurts me too
    // TODO: this could be something neater like a prefix tree I think, but for now this will do
    seen: HashMap<String, Vec<HashSet<String>>>,
    custom_metric_count: usize,
}

impl Tracker {
    fn new(count_threshold: usize, actions: Vec<ThresholdAction>) -> Self {
        let state = if !actions.is_empty() {
            TrackerState::Tracking(TrackingState {
                seen: HashMap::new(),
                custom_metric_count: 0,
            })
        } else {
            TrackerState::Done // If we won't do anything when we overrun the threshold, don't bother tracking
        };
        Self {
            count_threshold,
            actions: Mutex::from(actions),
            state: Mutex::new(state),
        }
    }

    pub(crate) fn track(&self, dd: &Datadog, metric: &str, tags: &[String]) {
        let mut state = self.state.lock().unwrap();
        // I know this is spooky, but it lets me move out of the mutex guard, which I want to do
        // so I have the option of moving the inner of TrackerState::Tracking into do_actions
        let old_state = std::mem::replace(&mut *state, TrackerState::Done);
        if let TrackerState::Tracking(mut inner) = old_state {
            self.update(&mut inner, metric, tags);
            if inner.custom_metric_count >= self.count_threshold {
                // We REALLY don't want to hold the lock while we run the actions, since a user
                // could e.g. call incr from within a custom action, and deadlock the whole app
                drop(state);
                return self.do_actions(dd, metric, tags, inner);
            }
            *state = TrackerState::Tracking(inner);
        }
    }

    fn update(&self, state: &mut TrackingState, metric: &str, tags: &[String]) {
        let seen_tag_sets = state.seen.entry(metric.to_string()).or_default();
        // This cloning is a little sad but necessary for the eq check we do next to be efficient.
        // We'd have to clone regardless if the tag set was novel, but that should be rare so it's still unfortunate.
        let new_tag_set = tags.iter().cloned().collect::<HashSet<_>>();
        let set_is_novel = seen_tag_sets.iter().all(|seen| !seen.eq(&new_tag_set));
        if set_is_novel {
            seen_tag_sets.push(new_tag_set);
            state.custom_metric_count += 1;
        }
    }

    fn do_actions(&self, dd: &Datadog, metric: &str, tags: &[String], state: TrackingState) {
        let mut actions = self.actions.lock().unwrap();
        let old_actions = std::mem::take(&mut *actions);
        for action in old_actions {
            match action {
                ThresholdAction::Event(title, text) => dd.do_event(title, text, self.generate_event_tags(&state)),
                ThresholdAction::Custom(mut action) => {
                    action(metric.to_string(), tags.to_owned());
                }
            }
        }
    }

    fn generate_event_tags(&self, state: &TrackingState) -> Vec<String> {
        state
            .seen
            .iter()
            .map(|(metric, unique_tag_sets)| format!("{}:{}", metric, unique_tag_sets.len()))
            .collect()
    }
}

// Actions that define what the tracker will do when the custom metric threshold is passed.
// A user may define any number of these, and by default none are taken.
enum ThresholdAction {
    // Emit an event. The events title is the first string, and the text the second. The count of unique tag sets,
    // per metric, is provided as the tags for the event, i.e. for a given metric "metric", there will be a tag "metric:count",
    // where count is the number of unique tag sets seen for that metric.
    Event(String, String),
    // Take some custom action. The function will be passed the metric name and tags
    Custom(Box<dyn FnMut(String, Vec<String>) + Send + Sync>),
}

pub struct TrackerConfiguration {
    count_threshold: usize,
    actions: Vec<ThresholdAction>,
}

impl Default for TrackerConfiguration {
    fn default() -> Self {
        Self {
            count_threshold: DEFAULT_TAG_THRESHOLD,
            actions: Vec::new(),
        }
    }
}

impl TrackerConfiguration {
    pub fn new() -> Self {
        Self {
            count_threshold: DEFAULT_TAG_THRESHOLD,
            actions: Vec::new(),
        }
    }

    pub fn with_event(mut self, title: String, text: String) -> Self {
        self.actions.push(ThresholdAction::Event(title, text));
        self
    }

    pub fn with_custom(mut self, custom_action: Box<dyn FnMut(String, Vec<String>) + Send + Sync>) -> Self {
        self.actions.push(ThresholdAction::Custom(custom_action));
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
