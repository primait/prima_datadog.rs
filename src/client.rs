use crate::{ServiceCheckOptions, ServiceStatus};

pub trait DogstatsdClient {
    /// Increment a StatsD counter
    fn incr(&self, metric: &str, tags: Vec<String>);

    /// Decrement a StatsD counter
    fn decr(&self, metric: &str, tags: Vec<String>);

    /// Make an arbitrary change to a StatsD counter
    fn count(&self, metric: &str, count: i64, tags: Vec<String>);

    /// Time how long it takes for a block of code to execute
    fn time(&self, metric: &str, tags: Vec<String>, block: Box<dyn FnOnce()>);

    /// Send your own timing metric in milliseconds
    fn timing(&self, metric: &str, ms: i64, tags: Vec<String>);

    /// Report an arbitrary value as a gauge
    fn gauge(&self, metric: &str, val: &str, tags: Vec<String>);

    /// Report a value in a histogram
    fn histogram(&self, metric: &str, val: &str, tags: Vec<String>);

    /// Report a value in a distribution
    fn distribution(&self, metric: &str, val: &str, tags: Vec<String>);

    /// Report a value in a set
    fn set(&self, metric: &str, val: &str, tags: Vec<String>);

    /// Report the status of a service
    fn service_check(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: Vec<String>,
        options: Option<ServiceCheckOptions>,
    );

    /// Send a custom event as a title and a body
    fn event(&self, title: &str, text: &str, tags: Vec<String>);
}

impl DogstatsdClient for dogstatsd::Client {
    fn incr(&self, metric: &str, tags: Vec<String>) {
        let _ = self.incr(metric, tags);
    }

    fn decr(&self, metric: &str, tags: Vec<String>) {
        let _ = self.decr(metric, tags);
    }

    fn count(&self, metric: &str, count: i64, tags: Vec<String>) {
        let _ = self.count(metric, count, tags);
    }

    fn time(&self, metric: &str, tags: Vec<String>, block: Box<dyn FnOnce()>) {
        let _ = self.time(metric, tags, block);
    }

    fn timing(&self, metric: &str, ms: i64, tags: Vec<String>) {
        let _ = self.timing(metric, ms, tags);
    }

    fn gauge(&self, metric: &str, val: &str, tags: Vec<String>) {
        let _ = self.gauge(metric, val, tags);
    }

    fn histogram(&self, metric: &str, val: &str, tags: Vec<String>) {
        let _ = self.histogram(metric, val, tags);
    }

    fn distribution(&self, metric: &str, val: &str, tags: Vec<String>) {
        let _ = self.distribution(metric, val, tags);
    }

    fn set(&self, metric: &str, val: &str, tags: Vec<String>) {
        let _ = self.set(metric, val, tags);
    }

    fn service_check(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: Vec<String>,
        options: Option<ServiceCheckOptions>,
    ) {
        let _ = self.service_check(metric, val, tags, options);
    }

    fn event(&self, title: &str, text: &str, tags: Vec<String>) {
        let _ = self.event(title, text, tags);
    }
}
