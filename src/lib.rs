use crate::client::DogstatsdClient;
use crate::configuration::Configuration;
use crate::error::Error;
use once_cell::sync::OnceCell;

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
        let datadog_instance = Self {
            client: Box::new(dogstatsd::Client::new(dogstatsd_client_options)?),
            is_reporting_enabled: configuration.is_reporting_enabled(),
            default_tags: vec![],
        };

        INSTANCE.get_or_init(|| datadog_instance);

        Ok(())
    }

    pub fn current() -> &'static Datadog {
        INSTANCE.get().expect("Datadog not initialized")
    }

    pub fn incr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        dbg!(&tags);
        self.client.incr(metric.as_ref(), tags);
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
        $crate::Datadog::current().incr($stat, vec![]);
    };
    ($stat:path) => {
        $crate::Datadog::current().incr($stat.as_ref(), vec![]);
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        let tags = std::vec![
            $(
                std::format!("{}:{}", $key, $value)
            ), *
        ];
        $crate::Datadog::current().incr($stat, tags);
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        let tags = std::vec![
            $(
                std::format!("{}:{}", $key, $value)
            ), *
        ];
        $crate::Datadog::current().incr($stat.as_ref(), tags);
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

    impl AsRef<str> for TestEvent {
        fn as_ref(&self) -> &str {
            match self {
                TestEvent::Test1 => "test1_event",
            }
        }
    }

    fn incr_mock(
        metric: &'static str,
        tags: &'static [&str],
    ) -> crate::client::MockDogstatsdClient {
        let mut client_mock = crate::client::MockDogstatsdClient::new();
        client_mock
            .expect_incr()
            .once()
            .with(
                eq(metric),
                mockall::predicate::function(move |called_tags: &Vec<String>| {
                    called_tags.iter().all(|tag| tags.contains(&tag.as_str()))
                }),
            )
            .return_const(());

        client_mock
    }

    #[test]
    pub fn incr_with_literal() {
        let mock = incr_mock("test", &[]);
        Datadog {
            client: Box::new(mock),
            is_reporting_enabled: true,
            default_tags: vec![],
        }
        .incr("test", vec![]);
    }

    #[test]
    pub fn incr_with_type() {
        let mock = incr_mock("test1_event", &[]);
        Datadog {
            client: Box::new(mock),
            is_reporting_enabled: true,
            default_tags: vec![],
        }
        .incr(TestEvent::Test1, vec![]);
    }

    #[test]
    pub fn incr_with_literal_and_tags() {
        let mock = incr_mock("test", &["added:tag", "test"]);
        Datadog {
            client: Box::new(mock),
            is_reporting_enabled: true,
            default_tags: vec!["test".to_string()],
        }
        .incr("test", vec!["added:tag".to_string()]);
    }

    #[test]
    pub fn incr_with_type_and_tags() {
        let mock = incr_mock("test1_event", &["added:tag", "test"]);
        Datadog {
            client: Box::new(mock),
            is_reporting_enabled: true,
            default_tags: vec!["test".to_string()],
        }
        .incr("test1_event", vec!["added:tag".to_string()]);
    }
}
