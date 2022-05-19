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
//! ```
//!
//! Then you can use the macros exposed at the base level of the module.
//! All macros accepts
//! - a string value or a path to a type that implements AsRef<str> as first argument.
//! - zero or more arguments, separated by comma `,`, for the metrics that needs more data.
//!     For example `count!` and `timing!` accepts a number while `service_check!` accepts a [ServiceStatus] and a [ServiceCheckOptions]
//! - a list of tags (which is separated from the rest of the arguments by semicolon `;`) in the form of `"name" => "value"`
//!
//! ```
//! # use prima_datadog::{*, configuration::PrimaConfiguration};
//! # let configuration = PrimaConfiguration::new(
//! #     "0.0.0.0:1234", // to address
//! #     "0.0.0.0:0", // from address
//! #     "service_name", // namespace for all metrics
//! #     "production".parse().unwrap() // environment
//! # );
//! # Datadog::init(configuration);
//! incr!("test");
//! # incr!("test"; "some" => "data");
//! # decr!("test");
//! decr!("test"; "some" => "data");
//! count!("test", 20);
//! count!("test", 10; "some" => "data");
//! time!("test", || { println!("expensive computation");});
//! time!("test", || { println!("expensive computation");}; "some" => "data");
//! # timing!("test", 20);
//! timing!("test", 20; "some" => "data");
//! # gauge!("test", "gauge value");
//! gauge!("test", "gauge value"; "some" => "data");
//! # histogram!("test", "histogram value");
//! histogram!("test", "histogram value"; "some" => "data");
//! # distribution!("test", "distribution value");
//! distribution!("test", "distribution value"; "some" => "data");
//! # set!("test", "set value");
//! set!("test", "set value"; "some" => "data");
//! service_check!("test", ServiceStatus::OK);
//! service_check!("test", ServiceStatus::OK, ServiceCheckOptions::default());
//! # event!("test", "test event");
//! event!("test", "test event"; "some" => "data");
//! ```
//!
//! This is an example of a custom metric, in this case based on an enum type, but it can really be whatever you want, as long as it implements AsRef<str>
//!
//! ```
//! # use prima_datadog::{*, configuration::PrimaConfiguration};
//! # let configuration = PrimaConfiguration::new(
//! #     "0.0.0.0:1234", // to address
//! #     "0.0.0.0:0", // from address
//! #     "service_name", // namespace for all metrics
//! #     "production".parse().unwrap() // environment
//! # );
//! # Datadog::init(configuration);
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

pub use dogstatsd::{ServiceCheckOptions, ServiceStatus};
use once_cell::sync::OnceCell;

pub use client::DogstatsdClient;
pub use macros::*;

use crate::configuration::Configuration;
use crate::error::Error;

mod client;
pub mod configuration;
pub mod error;
mod macros;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

static INSTANCE: OnceCell<Datadog> = OnceCell::new();

/// The Datadog struct is the main entry point for the library
pub struct Datadog {
    /// an instance of a dogstatsd::Client
    client: Box<dyn DogstatsdClient + Send + Sync>,
    /// tells if metric should be reported. If false, nothing is sent to the udp socket.
    is_reporting_enabled: bool,
}

impl Datadog {
    /// Initializes a Datadog instance with a struct that implements the [Configuration] trait.
    /// Make sure that you run it only once otherwise you will get an error.
    pub fn init(configuration: impl Configuration) -> Result<(), Error> {
        let mut initialized: bool = false;

        // the closure is guaranteed to execute only once
        let _ = INSTANCE.get_or_try_init::<_, Error>(|| {
            initialized = true;

            let dogstatsd_client_options: dogstatsd::Options = dogstatsd::Options::new(
                configuration.from_addr(),
                configuration.to_addr(),
                configuration.namespace(),
                configuration.default_tags(),
            );

            let client: dogstatsd::Client = dogstatsd::Client::new(dogstatsd_client_options)?;
            Ok(Self::new(client, configuration.is_reporting_enabled()))
        })?;

        if initialized {
            Ok(())
        } else {
            Err(Error::OnceCellAlreadyInitialized)
        }
    }

    /// initialize a Datadog instance with bare parameters. This should be used carefully.
    /// Consider using [Datadog::init] instead
    pub fn new(client: impl 'static + DogstatsdClient + Send + Sync, is_reporting_enabled: bool) -> Self {
        Self {
            client: Box::new(client),
            is_reporting_enabled,
        }
    }

    /// Increment a StatsD counter
    pub fn incr(metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._incr(metric.as_ref(), tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn _incr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            self.client
                .incr(metric.as_ref(), tags.into_iter().collect::<Vec<String>>());
        }
    }

    /// Decrement a StatsD counter
    pub fn decr(metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._decr(metric.as_ref(), tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn _decr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            self.client
                .decr(metric.as_ref(), tags.into_iter().collect::<Vec<String>>());
        }
    }

    /// Make an arbitrary change to a StatsD counter
    pub fn count(metric: impl AsRef<str>, count: i64, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._count(metric.as_ref(), count, tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn _count(&self, metric: impl AsRef<str>, count: i64, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            self.client
                .count(metric.as_ref(), count, tags.into_iter().collect::<Vec<String>>());
        }
    }

    /// Time a block of code (reports in ms)
    pub fn time(metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>, block: impl FnOnce() + 'static) {
        if let Some(instance) = INSTANCE.get() {
            instance._time(
                metric.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
                Box::new(block),
            );
        }
    }

    pub(crate) fn _time(
        &self,
        metric: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
        block: impl FnOnce() + 'static,
    ) {
        if self.is_reporting_enabled {
            self.client.time(
                metric.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
                Box::new(block),
            );
        }
    }

    /// Send your own timing metric in milliseconds
    pub fn timing(metric: impl AsRef<str>, ms: i64, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._timing(metric.as_ref(), ms, tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn _timing(&self, metric: impl AsRef<str>, ms: i64, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            self.client
                .timing(metric.as_ref(), ms, tags.into_iter().collect::<Vec<String>>());
        }
    }

    /// Report an arbitrary value as a gauge
    pub fn gauge(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._gauge(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn _gauge(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            self.client.gauge(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    /// Report a value in a histogram
    pub fn histogram(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._histogram(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn _histogram(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            self.client.histogram(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    /// Report a value in a distribution
    pub fn distribution(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._distribution(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn _distribution(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            self.client.distribution(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    /// Report a value in a set
    pub fn set(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._set(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn _set(&self, metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            self.client.set(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    /// Report the status of a service
    pub fn service_check(
        metric: impl AsRef<str>,
        value: ServiceStatus,
        tags: impl IntoIterator<Item = String>,
        options: Option<ServiceCheckOptions>,
    ) {
        if let Some(instance) = INSTANCE.get() {
            instance._service_check(
                metric.as_ref(),
                value,
                tags.into_iter().collect::<Vec<String>>(),
                options,
            );
        }
    }

    pub(crate) fn _service_check(
        &self,
        metric: impl AsRef<str>,
        value: ServiceStatus,
        tags: impl IntoIterator<Item = String>,
        options: Option<ServiceCheckOptions>,
    ) {
        if self.is_reporting_enabled {
            self.client.service_check(
                metric.as_ref(),
                value,
                tags.into_iter().collect::<Vec<String>>(),
                options,
            );
        }
    }

    /// Send a custom event as a title and a body
    pub fn event(metric: impl AsRef<str>, text: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance._event(
                metric.as_ref(),
                text.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn _event(
        &self,
        metric: impl AsRef<str>,
        text: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            self.client.event(
                metric.as_ref(),
                text.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }
}
