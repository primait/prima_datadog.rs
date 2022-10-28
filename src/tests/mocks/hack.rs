use super::*;

/// The `mock!` macro isn't very good with lifetimes or generics
/// so this is a wrapper around the mock that uses concrete types
pub(super) trait MockDogstatsdClient {
    fn incr(&self, metric: &str, tags: Vec<String>);
    fn decr(&self, metric: &str, tags: Vec<String>);
    fn count(&self, metric: &str, count: i64, tags: Vec<String>);
    fn time<'a>(&self, metric: &str, tags: Vec<String>, block: Box<dyn FnOnce() + 'a>);
    fn timing(&self, metric: &str, ms: i64, tags: Vec<String>);
    fn gauge(&self, metric: &str, val: &str, tags: Vec<String>);
    fn histogram(&self, metric: &str, val: &str, tags: Vec<String>);
    fn distribution(&self, metric: &str, val: &str, tags: Vec<String>);
    fn set(&self, metric: &str, val: &str, tags: Vec<String>);
    fn service_check(&self, metric: &str, val: ServiceStatus, tags: Vec<String>, options: Option<ServiceCheckOptions>);
    fn event(&self, title: &str, text: &str, tags: Vec<String>);
}
impl<C: MockDogstatsdClient> DogstatsdClient for C {
    fn incr(&self, metric: &str, tags: &[&str]) {
        MockDogstatsdClient::incr(self, metric, tags.iter().map(|s| s.to_string()).collect())
    }

    fn decr(&self, metric: &str, tags: &[&str]) {
        MockDogstatsdClient::decr(self, metric, tags.iter().map(|s| s.to_string()).collect())
    }

    fn count(&self, metric: &str, count: i64, tags: &[&str]) {
        MockDogstatsdClient::count(self, metric, count, tags.iter().map(|s| s.to_string()).collect())
    }

    fn time(&self, metric: &str, tags: &[&str], block: impl FnOnce()) {
        MockDogstatsdClient::time(
            self,
            metric,
            tags.iter().map(|s| s.to_string()).collect(),
            Box::new(block),
        )
    }

    fn timing(&self, metric: &str, ms: i64, tags: &[&str]) {
        MockDogstatsdClient::timing(self, metric, ms, tags.iter().map(|s| s.to_string()).collect())
    }

    fn gauge(&self, metric: &str, val: &str, tags: &[&str]) {
        MockDogstatsdClient::gauge(self, metric, val, tags.iter().map(|s| s.to_string()).collect())
    }

    fn histogram(&self, metric: &str, val: &str, tags: &[&str]) {
        MockDogstatsdClient::histogram(self, metric, val, tags.iter().map(|s| s.to_string()).collect())
    }

    fn distribution(&self, metric: &str, val: &str, tags: &[&str]) {
        MockDogstatsdClient::distribution(self, metric, val, tags.iter().map(|s| s.to_string()).collect())
    }

    fn set(&self, metric: &str, val: &str, tags: &[&str]) {
        MockDogstatsdClient::set(self, metric, val, tags.iter().map(|s| s.to_string()).collect())
    }

    fn service_check(&self, metric: &str, val: ServiceStatus, tags: &[&str], options: Option<ServiceCheckOptions>) {
        MockDogstatsdClient::service_check(self, metric, val, tags.iter().map(|s| s.to_string()).collect(), options)
    }

    fn event(&self, title: &str, text: &str, tags: &[&str]) {
        MockDogstatsdClient::event(self, title, text, tags.iter().map(|s| s.to_string()).collect())
    }
}
