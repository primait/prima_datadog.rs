use crate::{ServiceCheckOptions, ServiceStatus, TagsProvider};

/// This trait represent a client that is able to interact with the datadog statsd collector.
/// Its main use in this library is having a common interface for the underlying implementation,
/// and being able to mock it for testing purposes
pub trait DogstatsdClient {
    /// Increment a StatsD counter
    fn incr<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>);

    /// Decrement a StatsD counter
    fn decr<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>);

    /// Make an arbitrary change to a StatsD counter
    fn count<S: AsRef<str>>(&self, metric: &str, count: i64, tags: impl TagsProvider<S>);

    /// Time how long it takes for a block of code to execute
    fn time<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>, block: impl FnOnce());

    /// Send your own timing metric in milliseconds
    fn timing<S: AsRef<str>>(&self, metric: &str, ms: i64, tags: impl TagsProvider<S>);

    /// Report an arbitrary value as a gauge
    fn gauge<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>);

    /// Report a value in a histogram
    fn histogram<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>);

    /// Report a value in a distribution
    fn distribution<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>);

    /// Report a value in a set
    fn set<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>);

    /// Report the status of a service
    fn service_check<S: AsRef<str>>(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    );

    /// Send a custom event as a title and a body
    fn event<S: AsRef<str>>(&self, title: &str, text: &str, tags: impl TagsProvider<S>);
}

impl DogstatsdClient for dogstatsd::Client {
    fn incr<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>) {
        let _ = self.incr(metric, tags.as_ref());
    }

    fn decr<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>) {
        let _ = self.decr(metric, tags.as_ref());
    }

    fn count<S: AsRef<str>>(&self, metric: &str, count: i64, tags: impl TagsProvider<S>) {
        let _ = self.count(metric, count, tags.as_ref());
    }

    fn time<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>, block: impl FnOnce()) {
        let _ = self.time(metric, tags.as_ref(), block);
    }

    fn timing<S: AsRef<str>>(&self, metric: &str, ms: i64, tags: impl TagsProvider<S>) {
        let _ = self.timing(metric, ms, tags.as_ref());
    }

    fn gauge<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        let _ = self.gauge(metric, val, tags.as_ref());
    }

    fn histogram<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        let _ = self.histogram(metric, val, tags.as_ref());
    }

    fn distribution<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        let _ = self.distribution(metric, val, tags.as_ref());
    }

    fn set<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        let _ = self.set(metric, val, tags.as_ref());
    }

    fn service_check<S: AsRef<str>>(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    ) {
        let _ = self.service_check(metric, val, tags.as_ref(), options);
    }

    fn event<S: AsRef<str>>(&self, title: &str, text: &str, tags: impl TagsProvider<S>) {
        let _ = self.event(title, text, tags.as_ref());
    }
}
