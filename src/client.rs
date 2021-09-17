pub trait DogstatsdClient {
    /// Increment a StatsD counter
    fn incr(&self, metric: &str, tags: Vec<String>);

    /// Decrement a StatsD counter
    fn decr(&self, metric: &str, tags: Vec<String>);
}

impl DogstatsdClient for dogstatsd::Client {
    fn incr(&self, metric: &str, tags: Vec<String>) {
        let _ = self.incr(metric, tags);
    }

    fn decr(&self, metric: &str, tags: Vec<String>) {
        let _ = self.decr(metric, tags);
    }
}
