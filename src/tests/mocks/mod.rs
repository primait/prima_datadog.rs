use mockall::{mock, predicate::*};

use crate::*;

mod hack;
use hack::*;

mock! {
    pub Client {}
    impl MockDogstatsdClient for Client {
        /// Increment a StatsD counter
        fn incr(&self, metric: &str, tags: Vec<String>);

        /// Decrement a StatsD counter
        fn decr(&self, metric: &str, tags: Vec<String>);

        /// Make an arbitrary change to a StatsD counter
        fn count(&self, metric: &str, count: i64, tags: Vec<String>);

        /// Time how long it takes for a block of code to execute
        fn time<'a>(&self, metric: &str, tags: Vec<String>, block: Box<dyn FnOnce() + 'a>);

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
}

#[allow(dead_code)]
pub fn incr_mock(metric: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_incr()
        .once()
        .with(
            eq(metric),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn decr_mock(metric: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_decr()
        .once()
        .with(
            eq(metric),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn count_mock(metric: &'static str, count: i64, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_count()
        .once()
        .with(
            eq(metric),
            eq(count),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn time_mock(metric: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_time()
        .once()
        .with(
            eq(metric),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
            always(),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn timing_mock(metric: &'static str, ms: i64, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_timing()
        .once()
        .with(
            eq(metric),
            eq(ms),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn gauge_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_gauge()
        .once()
        .with(
            eq(metric),
            eq(value),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn histogram_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_histogram()
        .once()
        .with(
            eq(metric),
            eq(value),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn distribution_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_distribution()
        .once()
        .with(
            eq(metric),
            eq(value),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn set_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_set()
        .once()
        .with(
            eq(metric),
            eq(value),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn service_check_mock(
    metric: &'static str,
    value: ServiceStatus,
    tags: &'static [&str],
    options: Option<ServiceCheckOptions>,
) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_service_check()
        .once()
        .with(
            eq(metric),
            function(move |called_value: &ServiceStatus| {
                matches!(
                    (called_value, value),
                    (ServiceStatus::OK, ServiceStatus::OK)
                        | (ServiceStatus::Critical, ServiceStatus::Critical)
                        | (ServiceStatus::Unknown, ServiceStatus::Unknown)
                        | (ServiceStatus::Warning, ServiceStatus::Warning)
                )
            }),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
            function(move |called_options: &Option<ServiceCheckOptions>| {
                matches!((called_options, options), (Some(_), Some(_)) | (None, None))
            }),
        )
        .return_const(());

    client_mock
}

#[allow(dead_code)]
pub fn event_mock(metric: &'static str, text: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_event()
        .once()
        .with(
            eq(metric),
            eq(text),
            function(move |called_tags: &Vec<String>| called_tags.iter().all(|tag| tags.contains(&tag.as_str()))),
        )
        .return_const(());

    client_mock
}
