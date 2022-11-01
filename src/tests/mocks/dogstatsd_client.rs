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
    fn incr(&self, metric: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::incr(self, metric, tags.into_iter().map(|s| s.as_ref().to_string()).collect())
    }

    fn decr(&self, metric: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::decr(self, metric, tags.into_iter().map(|s| s.as_ref().to_string()).collect())
    }

    fn count(&self, metric: &str, count: i64, tags: impl TagsProvider) {
        MockDogstatsdClient::count(
            self,
            metric,
            count,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn time(&self, metric: &str, tags: impl TagsProvider, block: impl FnOnce()) {
        MockDogstatsdClient::time(
            self,
            metric,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
            Box::new(block),
        )
    }

    fn timing(&self, metric: &str, ms: i64, tags: impl TagsProvider) {
        MockDogstatsdClient::timing(
            self,
            metric,
            ms,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn gauge(&self, metric: &str, val: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::gauge(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn histogram(&self, metric: &str, val: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::histogram(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn distribution(&self, metric: &str, val: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::distribution(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn set(&self, metric: &str, val: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::set(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn service_check(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: impl TagsProvider,
        options: Option<ServiceCheckOptions>,
    ) {
        MockDogstatsdClient::service_check(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
            options,
        )
    }

    fn event(&self, title: &str, text: &str, tags: impl TagsProvider) {
        MockDogstatsdClient::event(
            self,
            title,
            text,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }
}
