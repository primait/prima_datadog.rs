use std::{convert::TryInto, time::Instant};

use crate::{Datadog, TagsProvider};

pub struct TimingGuard {
    start: Instant,
    tags: Vec<String>,
}

impl TimingGuard {
    pub(crate) fn new<S: AsRef<str>>(name: impl AsRef<str>, path: usize, tags: impl TagsProvider<S>) -> Self {
        let mut res = Self {
            start: Instant::now(),
            tags: tags.as_ref().iter().map(|t| t.as_ref().to_string()).collect(),
        };
        res.tags.push(format!("path_taken:{}", path));
        // All comparisons go under an "experiment" metric, and then we tag each comparison with the name of the experiment
        // This is to make it easy to browse all the different experiments in datadog by looking at the metric information
        res.tags.push(format!("name:{}", name.as_ref()));
        res
    }
}

impl Drop for TimingGuard {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        match elapsed.as_millis().try_into() {
            Ok(millis) => Datadog::timing("experiment", millis, &self.tags),
            Err(_) => {
                // TODO - is it possible to filter on this in datadog? If not, I might need to
                // always add an "overflowed" tag, and set it to "false" in cases where the conversion
                // succeeded, and "true" otherwise
                self.tags.push("overflowed".to_string());
                Datadog::timing("experiments", i64::MAX, &self.tags);
                return;
            }
        }
    }
}
