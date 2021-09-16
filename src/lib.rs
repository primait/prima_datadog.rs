use once_cell::sync::OnceCell;

use crate::client::DogstatsdClient;
use crate::configuration::Configuration;
use crate::error::Error;

mod client;
pub mod configuration;
pub mod error;

static INSTANCE: OnceCell<Datadog> = OnceCell::new();

pub struct Datadog {
    /// an instance of a dogstatsd::Client
    client: Box<dyn DogstatsdClient + Send + Sync>,
    /// tells if metric should be reported. If false, nothing is sent to the udp socket.
    is_reporting_enabled: bool,
    /// default tags that will be added to every reported metric
    default_tags: Vec<String>,
}

impl Datadog {
    pub fn init(configuration: impl Configuration) -> Result<(), Error> {
        let dogstatsd_client_options = dogstatsd::Options::new(
            configuration.from_addr(),
            configuration.to_addr(),
            configuration.namespace(),
        );
        INSTANCE
            .set(Self {
                client: Box::new(dogstatsd::Client::new(dogstatsd_client_options)?),
                is_reporting_enabled: configuration.is_reporting_enabled(),
                default_tags: vec![],
            })
            .map_err(|_| Error::OnceCell)?;

        Ok(())
    }

    pub fn global() -> &'static Datadog {
        INSTANCE.get().expect("Datadog is not initialized")
    }
}

impl DogstatsdClient for Datadog {
    fn incr(&self, metric: &str, tags: Vec<String>) {
        if !self.is_reporting_enabled {
            return;
        }
        let _ = self.client.incr(
            metric,
            tags.into_iter()
                .chain(self.default_tags.iter().cloned())
                .collect(),
        );
    }

    fn decr(&self, metric: &str, tags: Vec<String>) {
        if !self.is_reporting_enabled {
            return;
        }
        let _ = self.client.decr(
            metric,
            tags.into_iter()
                .chain(self.default_tags.iter().cloned())
                .collect(),
        );
    }
}

#[macro_export]
macro_rules! incr {
    ($stat:literal) => {
        $crate::Datadog::global().incr($stat, vec![]);
    };
    ($stat:path) => {
        $crate::Datadog::global().incr($stat.as_ref(), vec![]);
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        let tags = std::vec![
            $(
                std::format!("{}:{}", $key, $value)
            ), *
        ];
        $crate::Datadog::global().incr($stat, tags.into_iter());
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        let tags = std::vec![
            $(
                std::format!("{}:{}", $key, $value)
            ), *
        ];
        $crate::Datadog::global().incr($stat.as_ref(), tags.into_iter());
    };
}

#[macro_export]
macro_rules! decr {
    ($stat:literal) => {
        $crate::Datadog::global().decr($stat, std::iter::empty());
    };
    ($stat:path) => {
        $crate::Datadog::global().decr($stat.as_ref(), std::iter::empty());
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        let tags = std::vec![
            $(
                std::format!("{}:{}", $key, $value)
            ), *
        ];
        $crate::Datadog::global().decr($stat, tags.into_iter());
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        let tags = std::vec![
            $(
                std::format!("{}:{}", $key, $value)
            ), *
        ];
        $crate::Datadog::global().decr($stat.as_ref(), tags.into_iter());
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    enum TestEvent {
        Test1,
    }

    impl Datadog {
        pub fn for_test<C: 'static + DogstatsdClient + Send + Sync>(
            client: C,
            is_reporting_enabled: bool,
            default_tags: Vec<String>,
        ) {
            let _ = INSTANCE
                .set(Self {
                    client: Box::new(client),
                    is_reporting_enabled,
                    default_tags,
                })
                .ok()
                .unwrap();
        }
    }

    impl AsRef<str> for TestEvent {
        fn as_ref(&self) -> &str {
            match self {
                TestEvent::Test1 => "test1_event",
            }
        }
    }

    #[test]
    pub fn incr_macro_with_literal() {
        let mut client_mock = crate::client::MockDogstatsdClient::new();
        client_mock
            .expect_incr()
            .with(eq("pippo"), eq(vec!["test".to_string()]))
            .return_const(());
        Datadog::for_test(client_mock, true, vec!["test".to_string()]);

        incr!("pippo");
    }

    #[test]
    pub fn incr_macro_with_path() {
        let mut client_mock = crate::client::MockDogstatsdClient::new();
        client_mock
            .expect_incr()
            .with(eq("test1_event"), eq(vec!["test".to_string()]))
            .return_const(());
        Datadog::for_test(client_mock, true, vec!["test".to_string()]);
        incr!(TestEvent::Test1);
    }
}
