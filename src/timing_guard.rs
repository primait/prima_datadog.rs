use std::{convert::TryInto, time::Instant};

use crate::{Datadog, TagsProvider};

pub const EXPERIMENTS_METRIC_NAME: &str = "experiments";

/// A guard which emits a timing metric when dropped.
pub struct TimingGuard<S, P>
where
    S: AsRef<str>,
    P: TagsProvider<S>,
{
    name: String,
    start: Instant,
    tags: P,
    phantom: std::marker::PhantomData<S>,
}

impl<S, P> TimingGuard<S, P>
where
    S: AsRef<str>,
    P: TagsProvider<S>,
{
    pub(crate) fn new(name: impl AsRef<str>, tags: P) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            start: Instant::now(),
            tags,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<S, P> Drop for TimingGuard<S, P>
where
    S: AsRef<str>,
    P: TagsProvider<S>,
{
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        match elapsed.as_millis().try_into() {
            Ok(millis) => Datadog::timing(&self.name, millis, &self.tags),
            Err(_) => {
                let mut tags: Vec<_> = self.tags.as_ref().iter().map(|t| t.as_ref()).collect();
                tags.push("overflowed");
                Datadog::timing(EXPERIMENTS_METRIC_NAME, i64::MAX, tags);
            }
        }
    }
}
