//! This is an opinionated library to share code and approach to Datadog logging in prima.it
//!
//! ### Getting started
//!
//! You need to call [Datadog::init] in your main binary, and to do so you'll need as argument a type that implements the [Configuration] trait.
//!
//! Inside the [configuration] you'll find an [implementation of this trait][configuration::PrimaConfiguration] tailored for prima.it needs.
//!
//! ```
//! use prima_datadog::{Datadog, configuration::PrimaConfiguration};
//!
//! let configuration = PrimaConfiguration::new(
//!     "0.0.0.0:1234", // to address
//!     "0.0.0.0:0", // from address
//!     "service_name", // namespace for all metrics
//!     "staging" // environment
//! );
//! Datadog::init(configuration);
//! ```
//!
//! Then you can use the macros exposed at the base level of the module
//!
//! ```
//! prima_datadog::incr!("test");
//! prima_datadog::decr!("test"; "some" => "data");
//! ```
//!
//! The first argument is the metric name. It accepts string literal (like the previous example) or a type path that implements [AsRef<str>]
//!
//! ```
//! enum Metric {
//!     John,
//!     Paul,
//!     George,
//!     Ringo,
//! }
//!
//! impl AsRef<str> for Metric {
//!     fn as_ref(&self) -> &str {
//!         match self {
//!             Metric::John => "john",
//!             Metric::Paul => "paul",
//!             Metric::George => "george",
//!             Metric::Ringo => "ringo",
//!         }
//!     }
//! }
//! ```
//!
//! and then
//!
//! ```
//! prima_datadog::incr!(Metric::John; "play" => "guitar");
//! prima_datadog::incr!(Metric::Paul; "play" => "bass");
//! prima_datadog::incr!(Metric::George; "play" => "sitar");
//! prima_datadog::incr!(Metric::Ringo; "play" => "drums");
//! ```
//!
//! ## References
//!
//!   - [Datadog docs](https://docs.datadoghq.com/getting_started/)
//!   - [Getting started with Datadog tags](https://docs.datadoghq.com/getting_started/tagging/)

#![doc(issue_tracker_base_url = "https://github.com/primait/prima_datadog.rs/issues")]

use crate::configuration::Configuration;
use crate::error::Error;
use once_cell::sync::OnceCell;

mod client;
pub mod configuration;
pub mod error;

pub use client::DogstatsdClient;

static INSTANCE: OnceCell<Datadog> = OnceCell::new();

/// The Datadog struct is the main entry point for the library
pub struct Datadog {
    /// an instance of a dogstatsd::Client
    client: Box<dyn DogstatsdClient + Send + Sync>,
    /// tells if metric should be reported. If false, nothing is sent to the udp socket.
    is_reporting_enabled: bool,
    /// default tags that will be added to every reported metric
    default_tags: Vec<String>,
}

impl Datadog {
    /// Initializes a Datadog instance with a struct that implements the [Configuration] trait
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

    /// initialize a Datadog instance with bare parameters.
    /// This should be used carefully. Use [Datadog::init] instead
    pub fn new<'a>(
        client: impl 'static + DogstatsdClient + Send + Sync,
        is_reporting_enabled: bool,
        default_tags: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        Self {
            client: Box::new(client),
            is_reporting_enabled,
            default_tags: default_tags.into_iter().map(ToString::to_string).collect(),
        }
    }

    pub fn global() -> &'static Datadog {
        INSTANCE.get().expect("Datadog not initialized")
    }

    pub fn is_reporting_enabled(&self) -> bool {
        self.is_reporting_enabled
    }

    /// Increment a StatsD counter
    pub fn incr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.incr(metric.as_ref(), tags);
    }

    /// Decrement a StatsD counter
    pub fn decr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.decr(metric.as_ref(), tags);
    }
}

/// Increment a StatsD counter
#[macro_export]
macro_rules! incr {
    ($stat:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat, vec![]);
        }
    };
    ($stat:path) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat.as_ref(), vec![]);
        }
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().incr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

/// Decrement a StatsD counter
#[macro_export]
macro_rules! decr {
    ($stat:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().decr($stat, vec![]);
        }
    };
    ($stat:path) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().decr($stat.as_ref(), vec![]);
        }
    };
    ($stat:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().decr($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().decr($stat.as_ref(), std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
