use mockall::{mock, predicate::*};
use prima_datadog::DogstatsdClient;

mock! {
    pub Client {}
    impl DogstatsdClient for Client {
        /// Increment a StatsD counter
        fn incr(&self, metric: &str, tags: Vec<String>);

        /// Decrement a StatsD counter
        fn decr(&self, metric: &str, tags: Vec<String>);
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
            function(move |called_tags: &Vec<String>| {
                called_tags.iter().all(|tag| tags.contains(&tag.as_str()))
            }),
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
            function(move |called_tags: &Vec<String>| {
                called_tags.iter().all(|tag| tags.contains(&tag.as_str()))
            }),
        )
        .return_const(());

    client_mock
}
