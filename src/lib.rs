use dogstatsd::Client as DogstatsdClient;
use once_cell::sync::OnceCell;

use crate::configuration::Configuration;
use crate::error::Error;

pub mod configuration;
pub mod error;

static INSTANCE: OnceCell<Datadog> = OnceCell::new();

pub struct Datadog {
    /// an instance of a dogstatsd::Client
    client: DogstatsdClient,
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
                client: DogstatsdClient::new(dogstatsd_client_options)?,
                is_reporting_enabled: configuration.is_reporting_enabled(),
                default_tags: vec![],
            })
            .map_err(|_| Error::OnceCell)?;

        Ok(())
    }

    pub fn global() -> &'static Datadog {
        INSTANCE.get().expect("Datadog is not initialized")
    }

    /// Increment a StatsD counter
    pub fn incr(&self, metric: &str, tags: impl Iterator<Item = String>) {
        if !self.is_reporting_enabled {
            return;
        }
        let _ = self
            .client
            .incr(metric, tags.chain(self.default_tags.iter().cloned()));
    }

    /// Decrement a StatsD counter
    pub fn decr(&self, metric: &str, tags: impl Iterator<Item = String>) {
        if !self.is_reporting_enabled {
            return;
        }
        let _ = self
            .client
            .decr(metric, tags.chain(self.default_tags.iter().cloned()));
    }
}

#[macro_export]
macro_rules! incr {
    ($stat:literal) => {
        $crate::Datadog::global().incr($stat, std::iter::empty());
    };
    ($stat:path) => {
        $crate::Datadog::global().incr($stat.as_ref(), std::iter::empty());
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
