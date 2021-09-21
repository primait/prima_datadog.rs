//! This is an opinionated library to share code and approach to Datadog logging in prima.it
//!
//! ### Getting started
//!
//! You need to call [Datadog::init] in your main binary, and to do so you'll need as argument a type that implements the [Configuration] trait.
//!
//! Inside the [configuration] you'll find an [implementation of this trait][configuration::PrimaConfiguration] tailored for prima.it needs.
//!
//! ```
//! use prima_datadog::{*, configuration::PrimaConfiguration};
//!
//! // initializes the PrimaConfiguration struct
//! let configuration = PrimaConfiguration::new(
//!     "0.0.0.0:1234", // to address
//!     "0.0.0.0:0", // from address
//!     "service_name", // namespace for all metrics
//!     "production".parse().unwrap() // environment
//! );
//!
//! // Initializes a Datadog instance
//! Datadog::init(configuration);
//!
//! // Then you can use the macros exposed at the base level of the module
//! incr!("test");
//! incr!("test"; "some" => "data");
//! decr!("test");
//! decr!("test"; "some" => "data");
//! count!("test", 20);
//! count!("test", 10; "some" => "data");
//! time!("test", || { println!("expensive computation");});
//! time!("test", || { println!("expensive computation");}; "some" => "data");
//! timing!("test", 20);
//! timing!("test", 20; "some" => "data");
//! gauge!("test", "gauge value");
//! gauge!("test", "gauge value"; "some" => "data");
//! histogram!("test", "histogram value");
//! histogram!("test", "histogram value"; "some" => "data");
//! distribution!("test", "distribution value");
//! distribution!("test", "distribution value"; "some" => "data");
//! set!("test", "set value");
//! set!("test", "set value"; "some" => "data");
//!  
//! // The first argument is the metric name. It accepts string literal (like the previous example)
//! // or a type path that implements [AsRef] for `T: str`
//!
//! // custom metric, based on an enum type. It can really be whatever you want, //!
//! // as long as it implements AsRef<str>
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
//!
//! // now you can do
//! incr!(Metric::John; "play" => "guitar");
//! incr!(Metric::Paul; "play" => "bass");
//! incr!(Metric::George; "play" => "sitar");
//! incr!(Metric::Ringo; "play" => "drums");
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
pub use dogstatsd::{ServiceCheckOptions, ServiceStatus};

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
            default_tags: configuration.default_tags(),
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

    /// Make an arbitrary change to a StatsD counter
    pub fn count(
        &self,
        metric: impl AsRef<str>,
        count: i64,
        tags: impl IntoIterator<Item = String>,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.count(metric.as_ref(), count, tags);
    }

    /// Make an arbitrary change to a StatsD counter
    pub fn time(
        &self,
        metric: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
        block: impl FnOnce() + 'static,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.time(metric.as_ref(), tags, Box::new(block));
    }

    /// Send your own timing metric in milliseconds
    pub fn timing(&self, metric: impl AsRef<str>, ms: i64, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.timing(metric.as_ref(), ms, tags);
    }

    /// Report an arbitrary value as a gauge
    pub fn gauge(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.gauge(metric.as_ref(), value.as_ref(), tags);
    }

    /// Report a value in a histogram
    pub fn histogram(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.histogram(metric.as_ref(), value.as_ref(), tags);
    }

    /// Report a value in a distribution
    pub fn distribution(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client
            .distribution(metric.as_ref(), value.as_ref(), tags);
    }

    /// Report a value in a set
    pub fn set(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.set(metric.as_ref(), value.as_ref(), tags);
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

/// Decrement a StatsD counter
#[macro_export]
macro_rules! count {
    ($stat:literal, $count:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, vec![]);
        }
    };
    ($stat:path, $count:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, vec![]);
        }
    };
    ($stat:literal, $count:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat, $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $count:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().count($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

/// Make an arbitrary change to a StatsD counter
#[macro_export]
macro_rules! time {
    ($stat:literal, || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, vec![], || $block);
        }
    };
    ($stat:literal, move || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, vec![], || $block);
        }
    };
    ($stat:path, || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), vec![], || $block);
        }
    };
    ($stat:path, move || $block:expr) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), vec![], || $block);
        }
    };
    ($stat:literal, || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:literal, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:path, || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
    ($stat:path, move || $block:expr; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().time($stat.as_ref(), $count, std::vec![$(std::format!("{}:{}", $key, $value)), *], || $block);
        }
    };
}

/// Send your own timing metric in milliseconds
#[macro_export]
macro_rules! timing {
    ($stat:literal, $ms:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat, $ms, vec![]);
        }
    };
    ($stat:path, $ms:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat.as_ref(), $ms, vec![]);
        }
    };
    ($stat:literal, $ms:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat, $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $ms:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().timing($stat.as_ref(), $ms, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

/// Report an arbitrary value as a gauge
#[macro_export]
macro_rules! gauge {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().gauge($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

/// Report a value in a histogram
#[macro_export]
macro_rules! histogram {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().histogram($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().histogram($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().histogram($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().histogram($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

/// Report a value in a distribution
#[macro_export]
macro_rules! distribution {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().distribution($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}

/// Report a value in a set
#[macro_export]
macro_rules! set {
    ($stat:literal, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat, $val, vec![]);
        }
    };
    ($stat:path, $val:literal) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat.as_ref(), $val, vec![]);
        }
    };
    ($stat:literal, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat, $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
    ($stat:path, $val:literal; $( $key:expr => $value:expr ), *) => {
        if $crate::Datadog::global().is_reporting_enabled() {
            $crate::Datadog::global().set($stat.as_ref(), $val, std::vec![$(std::format!("{}:{}", $key, $value)), *]);
        }
    };
}
