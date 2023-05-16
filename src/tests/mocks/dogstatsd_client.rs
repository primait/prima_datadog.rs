use std::future::Future;

use async_trait::async_trait;

use super::*;

/// The `mock!` macro isn't very good with lifetimes or generics
/// so this is a wrapper around the mock that uses concrete types
pub(super) trait MockDogstatsdClient {
    fn incr(&self, metric: &str, tags: Vec<String>);
    fn decr(&self, metric: &str, tags: Vec<String>);
    fn count(&self, metric: &str, count: i64, tags: Vec<String>);
    fn time(&self, metric: &str, tags: Vec<String>);
    fn timing(&self, metric: &str, ms: i64, tags: Vec<String>);
    fn gauge(&self, metric: &str, val: &str, tags: Vec<String>);
    fn histogram(&self, metric: &str, val: &str, tags: Vec<String>);
    fn distribution(&self, metric: &str, val: &str, tags: Vec<String>);
    fn set(&self, metric: &str, val: &str, tags: Vec<String>);
    fn service_check(&self, metric: &str, val: ServiceStatus, tags: Vec<String>, options: Option<ServiceCheckOptions>);
    fn event(&self, title: &str, text: &str, tags: Vec<String>);
}

#[async_trait]
impl<C: MockDogstatsdClient + Sync> DogstatsdClient for C {
    fn incr<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::incr(
            self,
            metric,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn decr<S: AsRef<str>>(&self, metric: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::decr(
            self,
            metric,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn count<S: AsRef<str>>(&self, metric: &str, count: i64, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::count(
            self,
            metric,
            count,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn time<S, F, O>(&self, metric: &str, tags: impl TagsProvider<S>, block: F) -> O
    where
        S: AsRef<str>,
        F: FnOnce() -> O,
    {
        MockDogstatsdClient::time(
            self,
            metric,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        );
        block()
    }

    async fn async_time<S, F, T, O>(&self, metric: &str, tags: impl TagsProvider<S> + Send + Sync, block: F) -> O
    where
        S: AsRef<str> + Sync,
        F: FnOnce() -> T + Send,
        T: Future<Output = O> + Send,
    {
        MockDogstatsdClient::time(
            self,
            metric,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        );
        block().await
    }

    fn timing<S: AsRef<str>>(&self, metric: &str, ms: i64, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::timing(
            self,
            metric,
            ms,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn gauge<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::gauge(
            self,
            metric,
            val,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn histogram<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::histogram(
            self,
            metric,
            val,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn distribution<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::distribution(
            self,
            metric,
            val,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn set<S: AsRef<str>>(&self, metric: &str, val: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::set(
            self,
            metric,
            val,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn service_check<S: AsRef<str>>(
        &self,
        metric: &str,
        val: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    ) {
        MockDogstatsdClient::service_check(
            self,
            metric,
            val,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
            options,
        )
    }

    fn event<S: AsRef<str>>(&self, title: &str, text: &str, tags: impl TagsProvider<S>) {
        MockDogstatsdClient::event(
            self,
            title,
            text,
            tags.as_ref().iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }
}
