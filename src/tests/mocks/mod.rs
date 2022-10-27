use mockall::{mock, predicate::*};

use crate::*;

mock! {
    pub Client {}
    impl DogstatsdClient for Client {
        /// Increment a StatsD counter
        fn incr<'a>(&self, metric: &str, tags: &[&'a str]);

        /// Decrement a StatsD counter
        fn decr<'a>(&self, metric: &str, tags: &[&'a str]);

        /// Make an arbitrary change to a StatsD counter
        fn count<'a>(&self, metric: &str, count: i64, tags: &[&'a str]);

        /// Time how long it takes for a block of code to execute
        fn time<'a>(&self, metric: &str, tags: &[&'a str], block: Box<dyn FnOnce()>);

        /// Send your own timing metric in milliseconds
        fn timing<'a>(&self, metric: &str, ms: i64, tags: &[&'a str]);

        /// Report an arbitrary value as a gauge
        fn gauge<'a>(&self, metric: &str, val: &str, tags: &[&'a str]);

        /// Report a value in a histogram
        fn histogram<'a>(&self, metric: &str, val: &str, tags: &[&'a str]);

        /// Report a value in a distribution
        fn distribution<'a>(&self, metric: &str, val: &str, tags: &[&'a str]);

        /// Report a value in a set
        fn set<'a>(&self, metric: &str, val: &str, tags: &[&'a str]);

        /// Report the status of a service
        fn service_check<'a>(
            &self,
            metric: &str,
            val: ServiceStatus,
            tags: &[&'a str],
            options: Option<ServiceCheckOptions>,
        );

        /// Send a custom event as a title and a body
        fn event<'a>(&self, title: &str, text: &str, tags: &[&'a str]);
    }
}

#[allow(dead_code)]
pub fn incr_mock(metric: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_incr()
        .once()
        .return_once(move |called_metric: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        });

    client_mock
}

#[allow(dead_code)]
pub fn decr_mock(metric: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_decr()
        .once()
        .return_once(move |called_metric: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        });

    client_mock
}

#[allow(dead_code)]
pub fn count_mock(metric: &'static str, count: i64, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_count().once().return_once(
        move |called_metric: &str, called_count: i64, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(count, called_count);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

    client_mock
}

#[allow(dead_code)]
pub fn time_mock(metric: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_time().once().return_once(
        move |called_metric: &str, called_tags: &[&str], _block: Box<dyn FnOnce()>| {
            assert_eq!(metric, called_metric);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

    client_mock
}

#[allow(dead_code)]
pub fn timing_mock(metric: &'static str, ms: i64, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock
        .expect_timing()
        .once()
        .return_once(move |called_metric: &str, called_ms: i64, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(ms, called_ms);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        });

    client_mock
}

#[allow(dead_code)]
pub fn gauge_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_gauge().once().return_once(
        move |called_metric: &str, called_value: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(value, called_value);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

    client_mock
}

#[allow(dead_code)]
pub fn histogram_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_histogram().once().return_once(
        move |called_metric: &str, called_value: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(value, called_value);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

    client_mock
}

#[allow(dead_code)]
pub fn distribution_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_distribution().once().return_once(
        move |called_metric: &str, called_value: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(value, called_value);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

    client_mock
}

#[allow(dead_code)]
pub fn set_mock(metric: &'static str, value: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_set().once().return_once(
        move |called_metric: &str, called_value: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(value, called_value);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

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
    client_mock.expect_service_check().once().return_once(
        move |called_metric: &str,
              called_value: ServiceStatus,
              called_tags: &[&str],
              called_options: Option<ServiceCheckOptions>| {
            assert_eq!(metric, called_metric);
            assert!(matches!(
                (called_value, value),
                (ServiceStatus::OK, ServiceStatus::OK)
                    | (ServiceStatus::Critical, ServiceStatus::Critical)
                    | (ServiceStatus::Unknown, ServiceStatus::Unknown)
                    | (ServiceStatus::Warning, ServiceStatus::Warning)
            ));
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
            assert!(matches!((called_options, options), (Some(_), Some(_)) | (None, None)));
        },
    );

    client_mock
}

#[allow(dead_code)]
pub fn event_mock(metric: &'static str, text: &'static str, tags: &'static [&str]) -> MockClient {
    let mut client_mock = MockClient::new();
    client_mock.expect_event().once().return_once(
        move |called_metric: &str, called_text: &str, called_tags: &[&str]| {
            assert_eq!(metric, called_metric);
            assert_eq!(text, called_text);
            assert!(called_tags.iter().all(|tag| tags.contains(tag)), "tags don't match");
        },
    );

    client_mock
}
