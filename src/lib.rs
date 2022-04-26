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

use crate::configuration::Configuration;
use crate::error::Error;
use once_cell::sync::OnceCell;

mod client;
pub mod configuration;
pub mod error;
mod macros;

pub use client::DogstatsdClient;
pub use dogstatsd::{ServiceCheckOptions, ServiceStatus};
pub use macros::*;

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
    /// Initializes a Datadog instance with a struct that implements the [Configuration] trait.
    /// Make sure that you run it only once otherwise you will get an error.
    pub fn init(configuration: impl Configuration) -> Result<(), Error> {
        let mut initialized = false;

        // the closure is guaranteed to execute only once
        let _ = INSTANCE.get_or_try_init::<_, Error>(|| {
            initialized = true;
            let dogstatsd_client_options = dogstatsd::Options::new(
                configuration.from_addr(),
                configuration.to_addr(),
                configuration.namespace(),
                configuration.default_tags(),
            );

            Ok(Self {
                client: Box::new(dogstatsd::Client::new(dogstatsd_client_options)?),
                is_reporting_enabled: configuration.is_reporting_enabled(),
                default_tags: configuration.default_tags(),
            })
        })?;

        if initialized {
            Ok(())
        } else {
            Err(Error::OnceCellAlreadyInitialized)
        }
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
    pub fn count(&self, metric: impl AsRef<str>, count: i64, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.count(metric.as_ref(), count, tags);
    }

    /// Time a block of code (reports in ms)
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
    pub fn gauge(&self, metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.gauge(metric.as_ref(), value.as_ref(), tags);
    }

    /// Report a value in a histogram
    pub fn histogram(&self, metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
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
        self.client.distribution(metric.as_ref(), value.as_ref(), tags);
    }

    /// Report a value in a set
    pub fn set(&self, metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.set(metric.as_ref(), value.as_ref(), tags);
    }

    /// Report the status of a service
    pub fn service_check(
        &self,
        metric: impl AsRef<str>,
        value: ServiceStatus,
        tags: impl IntoIterator<Item = String>,
        options: Option<ServiceCheckOptions>,
    ) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.service_check(metric.as_ref(), value, tags, options);
    }

    /// Send a custom event as a title and a body
    pub fn event(&self, metric: impl AsRef<str>, text: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        let tags: Vec<String> = tags.into_iter().chain(self.default_tags.clone()).collect();
        self.client.event(metric.as_ref(), text.as_ref(), tags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{Environment, PrimaConfiguration};

    #[test]
    pub fn double_initialization() {
        let datadog = Datadog::init(PrimaConfiguration::new(
            "10.1.2.3:8125",
            "127.0.0.1:9000",
            "",
            Environment::Dev,
        ));
        assert!(datadog.is_ok());
        let datadog2 = Datadog::init(PrimaConfiguration::new(
            "10.1.2.3:8125",
            "127.0.0.1:9000",
            "",
            Environment::Production,
        ));
        assert!(datadog2.err().unwrap().is_once_cell_already_initialized());
    }
}
