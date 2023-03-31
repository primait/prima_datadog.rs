use crate::{ServiceCheckOptions, ServiceStatus, TagsProvider};

/// This trait represent a client that is able to interact with the datadog statsd collector.
/// Its main use in this library is having a common interface for the underlying implementation,
/// and being able to mock it for testing purposes
pub trait DogstatsdClient {
    /// Increment a StatsD counter
    fn incr<S>(&self, metric: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Decrement a StatsD counter
    fn decr<S>(&self, metric: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Make an arbitrary change to a StatsD counter
    fn count<S>(&self, metric: &str, count: i64, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Time how long it takes for a block of code to execute
    fn time<S>(&self, metric: &str, tags: impl TagsProvider<S>, block: impl FnOnce()) where S: AsRef<str>;

    /// Send your own timing metric in milliseconds
    fn timing<S>(&self, metric: &str, ms: i64, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Report an arbitrary value as a gauge
    fn gauge<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Report a value in a histogram
    fn histogram<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Report a value in a distribution
    fn distribution<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Report a value in a set
    fn set<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;

    /// Report the status of a service
    fn service_check<S>(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    ) where S: AsRef<str>;

    /// Send a custom event as a title and a body
    fn event<S>(&self, title: &str, text: &str, tags: impl TagsProvider<S>) where S: AsRef<str>;
}

impl DogstatsdClient for dogstatsd::Client {
    fn incr<S>(&self, metric: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.incr(metric, tags.as_ref());
    }

    fn decr<S>(&self, metric: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.decr(metric, tags.as_ref());
    }

    fn count<S>(&self, metric: &str, count: i64, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.count(metric, count, tags.as_ref());
    }

    fn time<S>(&self, metric: &str, tags: impl TagsProvider<S>, block: impl FnOnce()) where S: AsRef<str> {
        let _ = self.time(metric, tags.as_ref(), block);
    }

    fn timing<S>(&self, metric: &str, ms: i64, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.timing(metric, ms, tags.as_ref());
    }

    fn gauge<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.gauge(metric, val, tags.as_ref());
    }

    fn histogram<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.histogram(metric, val, tags.as_ref());
    }

    fn distribution<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.distribution(metric, val, tags.as_ref());
    }

    fn set<S>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.set(metric, val, tags.as_ref());
    }

    fn service_check<S>(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    ) where S: AsRef<str> {
        let _ = self.service_check(metric, val, tags.as_ref(), options);
    }

    fn event<S>(&self, title: &str, text: &str, tags: impl TagsProvider<S>) where S: AsRef<str> {
        let _ = self.event(title, text, tags.as_ref());
    }
}
