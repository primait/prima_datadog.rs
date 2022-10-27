use crate::{ServiceCheckOptions, ServiceStatus};

/// This trait represent a client that is able to interact with the datadog statsd collector.
/// Its main use in this library is having a common interface for the underlying implementation,
/// and being able to mock it for testing purposes
pub trait DogstatsdClient {
    /// Increment a StatsD counter
    fn incr<S, I>(&self, metric: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Decrement a StatsD counter
    fn decr<S, I>(&self, metric: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Make an arbitrary change to a StatsD counter
    fn count<S, I>(&self, metric: &str, count: i64, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Time how long it takes for a block of code to execute
    fn time<S, I>(&self, metric: &str, tags: I, block: impl FnOnce())
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Send your own timing metric in milliseconds
    fn timing<S, I>(&self, metric: &str, ms: i64, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Report an arbitrary value as a gauge
    fn gauge<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Report a value in a histogram
    fn histogram<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Report a value in a distribution
    fn distribution<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Report a value in a set
    fn set<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Report the status of a service
    fn service_check<S, I>(&self, metric: &str, val: ServiceStatus, tags: I, options: Option<ServiceCheckOptions>)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;

    /// Send a custom event as a title and a body
    fn event<S, I>(&self, title: &str, text: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>;
}

impl DogstatsdClient for dogstatsd::Client {
    fn incr<S, I>(&self, metric: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.incr(metric, tags);
    }

    fn decr<S, I>(&self, metric: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.decr(metric, tags);
    }

    fn count<S, I>(&self, metric: &str, count: i64, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.count(metric, count, tags);
    }

    fn time<S, I>(&self, metric: &str, tags: I, block: impl FnOnce())
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.time(metric, tags, block);
    }

    fn timing<S, I>(&self, metric: &str, ms: i64, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.timing(metric, ms, tags);
    }

    fn gauge<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.gauge(metric, val, tags);
    }

    fn histogram<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.histogram(metric, val, tags);
    }

    fn distribution<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.distribution(metric, val, tags);
    }

    fn set<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.set(metric, val, tags);
    }

    fn service_check<S, I>(&self, metric: &str, val: ServiceStatus, tags: I, options: Option<ServiceCheckOptions>)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.service_check(metric, val, tags, options);
    }

    fn event<S, I>(&self, title: &str, text: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        let _ = self.event(title, text, tags);
    }
}
