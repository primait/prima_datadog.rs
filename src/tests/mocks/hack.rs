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
    fn incr<S, I>(&self, metric: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::incr(self, metric, tags.into_iter().map(|s| s.as_ref().to_string()).collect())
    }

    fn decr<S, I>(&self, metric: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::decr(self, metric, tags.into_iter().map(|s| s.as_ref().to_string()).collect())
    }

    fn count<S, I>(&self, metric: &str, count: i64, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::count(
            self,
            metric,
            count,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn time<S, I>(&self, metric: &str, tags: I, block: impl FnOnce())
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::time(
            self,
            metric,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
            Box::new(block),
        )
    }

    fn timing<S, I>(&self, metric: &str, ms: i64, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::timing(
            self,
            metric,
            ms,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn gauge<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::gauge(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn histogram<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::histogram(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn distribution<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::distribution(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn set<S, I>(&self, metric: &str, val: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::set(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }

    fn service_check<S, I>(&self, metric: &str, val: ServiceStatus, tags: I, options: Option<ServiceCheckOptions>)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::service_check(
            self,
            metric,
            val,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
            options,
        )
    }

    fn event<S, I>(&self, title: &str, text: &str, tags: I)
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
    {
        MockDogstatsdClient::event(
            self,
            title,
            text,
            tags.into_iter().map(|s| s.as_ref().to_string()).collect(),
        )
    }
}
