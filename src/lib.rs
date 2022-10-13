//! This is an opinionated library to share code and approach to Datadog logging in prima.it
//!
//! ### Getting started
//!
//! You need to call [Datadog::init] in your main binary, and to do so you'll need as argument a type that implements the [Configuration] trait.
//! If you never call [Datadog::init] in your binary NO metrics will be sent.
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
//! ## Note - Avoid high tag cardinality!
//! It's important to avoid passing a large number of values for a given tag, as Datadog tracks each
//! unique combination of tag values as a separate metric, which can significantly impact billing.
//! For example, avoid passing things like user IDs, session IDs, request IDs, or other values that
//! vary significantly. See https://docs.datadoghq.com/getting_started/tagging/ for more information.
//!
//! By default, prima_datadog will emit an event if a large number of unique tag values are seen, with
//! the event title "prima_datadog_rs_tag_limit_exceeded". The threshold for this event to be emitted
//! default to 100. Different thresholds may be set using PrimaConfiguration::with_tag_warn_threshold.
//!
//! ## References
//!
//!   - [Datadog docs](https://docs.datadoghq.com/getting_started/)
//!   - [Getting started with Datadog tags](https://docs.datadoghq.com/getting_started/tagging/)
#![doc(issue_tracker_base_url = "https://github.com/primait/prima_datadog.rs/issues")]

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::Hasher;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

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
const TAG_LIMIT_EXCEEDED: &str = "prima_datadog_rs_tag_limit_exceeded";
/// See https://www.datadoghq.com/pricing/ and https://docs.datadoghq.com/account_management/billing/custom_metrics/,
/// 100 seems like a reasonable place to start warning for now
pub const DEFAULT_TAG_THRESHOLD: usize = 100;

/// The Datadog struct is the main entry point for the library
pub struct Datadog {
    /// an instance of a dogstatsd::Client
    client: Box<dyn DogstatsdClient + Send + Sync>,
    /// tells if metric should be reported. If false, nothing is sent to the udp socket.
    is_reporting_enabled: bool,
    // Tracking for high tag cardinality
    tag_tracker: TagTracker,
}

#[derive(Debug)]
struct TagTracker {
    // Store the set of tags we've seen for each metric, for reporting reasons
    seen_metrics: Mutex<HashMap<String, HashSet<u64>>>,
    // Store the total count of all metric:tag pairs
    total_count: AtomicUsize,
    // The limit of tag cardinality before we emit an event
    warn_threshold: usize,
    // We only want to send the warning event once, so we track whether we've sent it
    sent_event: AtomicBool,
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
            Ok(Self::new(
                client,
                configuration.is_reporting_enabled(),
                configuration.tag_warn_threshold(),
            ))
        })?;

        if initialized {
            Ok(())
        } else {
            Err(Error::OnceCellAlreadyInitialized)
        }
    }

    fn new(
        client: impl 'static + DogstatsdClient + Send + Sync,
        is_reporting_enabled: bool,
        tag_warn_threshold: usize,
    ) -> Self {
        Self {
            client: Box::new(client),
            is_reporting_enabled,
            tag_tracker: TagTracker {
                seen_metrics: Mutex::new(HashMap::new()),
                total_count: AtomicUsize::new(0),
                warn_threshold: tag_warn_threshold,
                sent_event: AtomicBool::new(false),
            },
        }
    }

    /// Increment a StatsD counter
    pub fn incr(metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_incr(metric.as_ref(), tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn do_incr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.incr(metric, tags);
        }
    }

    /// Decrement a StatsD counter
    pub fn decr(metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_decr(metric.as_ref(), tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn do_decr(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.decr(metric, tags);
        }
    }

    /// Make an arbitrary change to a StatsD counter
    pub fn count(metric: impl AsRef<str>, count: i64, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_count(metric.as_ref(), count, tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn do_count(&self, metric: impl AsRef<str>, count: i64, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.count(metric, count, tags);
        }
    }

    /// Time a block of code (reports in ms)
    pub fn time(metric: impl AsRef<str>, tags: impl IntoIterator<Item = String>, block: impl FnOnce() + 'static) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_time(
                metric.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
                Box::new(block),
            );
        }
    }

    pub(crate) fn do_time(
        &self,
        metric: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
        block: impl FnOnce() + 'static,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.time(metric, tags, Box::new(block));
        }
    }

    /// Send your own timing metric in milliseconds
    pub fn timing(metric: impl AsRef<str>, ms: i64, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_timing(metric.as_ref(), ms, tags.into_iter().collect::<Vec<String>>());
        }
    }

    pub(crate) fn do_timing(&self, metric: impl AsRef<str>, ms: i64, tags: impl IntoIterator<Item = String>) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.timing(metric, ms, tags);
        }
    }

    /// Report an arbitrary value as a gauge
    pub fn gauge(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_gauge(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn do_gauge(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.gauge(metric, value.as_ref(), tags);
        }
    }

    /// Report a value in a histogram
    pub fn histogram(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_histogram(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn do_histogram(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.histogram(metric, value.as_ref(), tags);
        }
    }

    /// Report a value in a distribution
    pub fn distribution(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_distribution(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn do_distribution(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.distribution(metric, value.as_ref(), tags);
        }
    }

    /// Report a value in a set
    pub fn set(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_set(
                metric.as_ref(),
                value.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn do_set(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.set(metric, value.as_ref(), tags);
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
            instance.do_service_check(
                metric.as_ref(),
                value,
                tags.into_iter().collect::<Vec<String>>(),
                options,
            );
        }
    }

    pub(crate) fn do_service_check(
        &self,
        metric: impl AsRef<str>,
        value: ServiceStatus,
        tags: impl IntoIterator<Item = String>,
        options: Option<ServiceCheckOptions>,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.service_check(metric, value, tags, options);
        }
    }

    /// Send a custom event as a title and a body
    pub fn event(metric: impl AsRef<str>, text: impl AsRef<str>, tags: impl IntoIterator<Item = String>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_event(
                metric.as_ref(),
                text.as_ref(),
                tags.into_iter().collect::<Vec<String>>(),
            );
        }
    }

    pub(crate) fn do_event(
        &self,
        metric: impl AsRef<str>,
        text: impl AsRef<str>,
        tags: impl IntoIterator<Item = String>,
    ) {
        if self.is_reporting_enabled {
            let tags = tags.into_iter().collect::<Vec<String>>();
            let metric = metric.as_ref();
            self.track_tags(metric, &tags);
            self.client.event(metric, text.as_ref(), tags);
        }
    }

    fn track_tags<'a>(&self, metric: impl AsRef<str>, tags: impl IntoIterator<Item = &'a String>) {
        let hashes = tags
            .into_iter()
            .map(|t| {
                let mut hasher = DefaultHasher::new();
                hasher.write(t.as_bytes());
                hasher.finish()
            })
            .collect::<Vec<u64>>(); // Collect the hashes to avoid doing the hashing work inside the locked section
        let delta = {
            let mut metrics = self.tag_tracker.seen_metrics.lock().unwrap();
            let tags = metrics.entry(metric.as_ref().to_string()).or_insert_with(HashSet::new);
            let before = tags.len();
            tags.extend(hashes);
            tags.len() - before
        };
        let prev = self.tag_tracker.total_count.fetch_add(delta, Ordering::SeqCst);
        if prev + delta >= self.tag_tracker.warn_threshold
            && self
                .tag_tracker
                .sent_event
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
        {
            let message = {
                let mut message = String::from("Exceeded tag cardinality limit. Metrics: ");
                let metrics = self.tag_tracker.seen_metrics.lock().unwrap();
                for entry in metrics.iter() {
                    message += &format!("{}: {}, ", entry.0, entry.1.len());
                }
                // Remove the trailing comma and space. Safety: we know there is at least one metric or we wouldn't be over the threshold
                message.truncate(message.len() - 2);
                message
            };
            println!("{}", message);
            self.do_event(String::from(TAG_LIMIT_EXCEEDED), message, vec![])
        }
    }
}
