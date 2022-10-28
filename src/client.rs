use crate::{ServiceCheckOptions, ServiceStatus};

/// This trait represent a client that is able to interact with the datadog statsd collector.
/// Its main use in this library is having a common interface for the underlying implementation,
/// and being able to mock it for testing purposes
pub trait DogstatsdClient {
    /// Increment a StatsD counter
    fn incr(&self, metric: &str, tags: &[&str]);

    /// Decrement a StatsD counter
    fn decr(&self, metric: &str, tags: &[&str]);

    /// Make an arbitrary change to a StatsD counter
    fn count(&self, metric: &str, count: i64, tags: &[&str]);

    /// Time how long it takes for a block of code to execute
    fn time(&self, metric: &str, tags: &[&str], block: impl FnOnce());

    /// Send your own timing metric in milliseconds
    fn timing(&self, metric: &str, ms: i64, tags: &[&str]);

    /// Report an arbitrary value as a gauge
    fn gauge(&self, metric: &str, val: &str, tags: &[&str]);

    /// Report a value in a histogram
    fn histogram(&self, metric: &str, val: &str, tags: &[&str]);

    /// Report a value in a distribution
    fn distribution(&self, metric: &str, val: &str, tags: &[&str]);

    /// Report a value in a set
    fn set(&self, metric: &str, val: &str, tags: &[&str]);

    /// Report the status of a service
    fn service_check(&self, metric: &str, val: ServiceStatus, tags: &[&str], options: Option<ServiceCheckOptions>);

    /// Send a custom event as a title and a body
    fn event(&self, title: &str, text: &str, tags: &[&str]);
}

impl DogstatsdClient for dogstatsd::Client {
    fn incr(&self, metric: &str, tags: &[&str]) {
        let _ = self.incr(metric, tags);
    }

    fn decr(&self, metric: &str, tags: &[&str]) {
        let _ = self.decr(metric, tags);
    }

    fn count(&self, metric: &str, count: i64, tags: &[&str]) {
        let _ = self.count(metric, count, tags);
    }

    fn time(&self, metric: &str, tags: &[&str], block: impl FnOnce()) {
        let _ = self.time(metric, tags, block);
    }

    fn timing(&self, metric: &str, ms: i64, tags: &[&str]) {
        let _ = self.timing(metric, ms, tags);
    }

    fn gauge(&self, metric: &str, val: &str, tags: &[&str]) {
        let _ = self.gauge(metric, val, tags);
    }

    fn histogram(&self, metric: &str, val: &str, tags: &[&str]) {
        let _ = self.histogram(metric, val, tags);
    }

    fn distribution(&self, metric: &str, val: &str, tags: &[&str]) {
        let _ = self.distribution(metric, val, tags);
    }

    fn set(&self, metric: &str, val: &str, tags: &[&str]) {
        let _ = self.set(metric, val, tags);
    }

    fn service_check(&self, metric: &str, val: ServiceStatus, tags: &[&str], options: Option<ServiceCheckOptions>) {
        let _ = self.service_check(metric, val, tags, options);
    }

    fn event(&self, title: &str, text: &str, tags: &[&str]) {
        let _ = self.event(title, text, tags);
    }
}
