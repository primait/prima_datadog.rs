//! This is an opinionated library to share code and approach to Datadog logging in prima.it
//!
//! ### Getting started
//!
//! You need to call [Datadog::init] in your main binary, and to do so you'll need as argument a type that implements the [Configuration] trait.
//! If you never call [Datadog::init] in your binary NO metrics will be sent.
//!
//! Inside the [configuration] you'll find an [implementation of this trait][configuration::Configuration] tailored for prima.it needs.
//!
//! ```
//! use prima_datadog::{*, configuration::Configuration};
//!
//! // initializes the Configuration struct
//! let configuration = Configuration::new(
//!     "0.0.0.0:1234", // to address
//!     "namespace", // namespace for all metrics
//! );
//!
//! // Initializes a Datadog instance
//! Datadog::init(configuration).unwrap();
//! ```
//!
//! Then you can use the macros exposed at the base level of the module.
//! All macros accepts
//! - a string value or a path to a type that implements `AsRef<str>` as first argument.
//! - zero or more arguments, separated by comma `,`, for the metrics that needs more data.
//!     For example `count!` and `timing!` accepts a number while `service_check!` accepts a [ServiceStatus] and a [ServiceCheckOptions]
//! - a list of tags (which is separated from the rest of the arguments by semicolon `;`) in the form of `"name" => "value"`
//!
//! ```
//! # use prima_datadog::{*, configuration::Configuration};
//! # let configuration = Configuration::new(
//! #     "0.0.0.0:1234", // to address
//! #     "namespace", // namespace for all metrics
//! # );
//! # Datadog::init(configuration).unwrap();
//!
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
//! # event!("test", "test event", EventOptions::new());
//! # event!("test", "test event", EventOptions::new(); "some" => "data");
//! ```
//!
//! This is an example of a custom metric, in this case based on an enum type, but it can really be
//! whatever you want, as long as it implements `AsRef<str>`.
//!
//! ```
//! # use prima_datadog::{*, configuration::Configuration};
//! # let configuration = Configuration::new(
//! #     "0.0.0.0:1234", // to address
//! #     "namespace", // namespace for all metrics
//! # );
//! # Datadog::init(configuration).unwrap();
//!
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
//! vary significantly. See <https://docs.datadoghq.com/getting_started/tagging/> for more information.
//!
//! Users may configure some actions to be taken when a metric cardinality threshold is exceeded. See
//! [TagTrackerConfiguration] for more information.
//!
//! ## References
//!
//!   - [Datadog docs](https://docs.datadoghq.com/getting_started/)
//!   - [Getting started with Datadog tags](https://docs.datadoghq.com/getting_started/tagging/)
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(issue_tracker_base_url = "https://github.com/primait/prima_datadog.rs/issues")]

use std::future::Future;

use configuration::Configuration;
pub use dogstatsd::{EventAlertType, EventOptions, ServiceCheckOptions, ServiceStatus};
use once_cell::sync::OnceCell;

pub use client::DogstatsdClient;
pub use tracker::*;

use crate::error::Error;

mod client;
pub mod configuration;
pub mod error;
mod macros;
pub mod timing_guard;
pub mod tracker;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;

/// Types that can provide an iterator of tags for a metric.
///
/// This is automatically implemented for anything that implements `AsRef<[S]>` where `S: AsRef<str>`.
///
/// In other words, you can pass a `&[&str]` (best), `Vec<&str>`, `&[String]`, `Vec<String>`, `&[Cow<str>]`, `Vec<Cow<str>>`, etc.
///
/// **If you'd like to pass in empty tags, use the [EMPTY_TAGS] constant.**
pub trait TagsProvider<S>
where
    Self: AsRef<[S]>,
    S: AsRef<str>,
{
}
impl<S, T> TagsProvider<S> for T
where
    T: AsRef<[S]>,
    S: AsRef<str>,
{
}

/// Helper constant for passing no tags to a metric.
///
/// This is a workaround for passing in `&[]` as a tag list, which is not
/// possible due to type inference rules as the compiler can't work out
/// the type inside the empty slice.
pub const EMPTY_TAGS: &[&str] = &[];

static INSTANCE: OnceCell<Datadog<dogstatsd::Client>> = OnceCell::new();

/// The Datadog type is the main entry point for the library
pub struct Datadog<C: DogstatsdClient> {
    /// an instance of a dogstatsd::Client
    inner: C,
    /// Tracking for high tag cardinality
    tag_tracker: Tracker,
}

impl Datadog<dogstatsd::Client> {
    /// Initializes a Datadog instance with a struct that implements the [Configuration] trait.
    /// Make sure that you run it only once otherwise you will get an error.
    pub fn init(mut configuration: Configuration) -> Result<(), Error> {
        let mut initialized: bool = false;

        // the closure is guaranteed to execute only once
        let _ = INSTANCE.get_or_try_init::<_, Error>(|| {
            initialized = true;

            let tracker_config = configuration.take_tracker_config();
            let dogstatsd_client_options: dogstatsd::Options = configuration.into();

            let client: dogstatsd::Client = dogstatsd::Client::new(dogstatsd_client_options)?;
            Ok(Self::new(client, tracker_config))
        })?;

        if initialized {
            Ok(())
        } else {
            Err(Error::OnceCellAlreadyInitialized)
        }
    }

    /// Increment a StatsD counter
    pub fn incr<S: AsRef<str>>(metric: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_incr(metric.as_ref(), tags);
        }
    }

    /// Decrement a StatsD counter
    pub fn decr<S: AsRef<str>>(metric: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_decr(metric.as_ref(), tags);
        }
    }

    /// Make an arbitrary change to a StatsD counter
    pub fn count<S: AsRef<str>>(metric: impl AsRef<str>, count: i64, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_count(metric.as_ref(), count, tags);
        }
    }

    /// Time a block of code (reports in ms)
    pub fn time<S, F, O>(metric: impl AsRef<str>, tags: impl TagsProvider<S>, block: F) -> O
    where
        S: AsRef<str>,
        F: FnOnce() -> O,
    {
        if let Some(instance) = INSTANCE.get() {
            instance.do_time(metric.as_ref(), tags, block)
        } else {
            block()
        }
    }

    pub async fn async_time<S, F, T, O>(metric: &str, tags: impl TagsProvider<S> + Send + Sync, block: F) -> O
    where
        S: AsRef<str> + Sync,
        F: FnOnce() -> T + Send,
        T: Future<Output = O> + Send,
    {
        if let Some(instance) = INSTANCE.get() {
            instance.do_async_time(metric.as_ref(), tags, block).await
        } else {
            block().await
        }
    }

    /// Send your own timing metric in milliseconds
    pub fn timing<S: AsRef<str>>(metric: impl AsRef<str>, ms: i64, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_timing(metric.as_ref(), ms, tags);
        }
    }

    /// Report an arbitrary value as a gauge
    pub fn gauge<S: AsRef<str>>(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_gauge(metric.as_ref(), value.as_ref(), tags);
        }
    }

    /// Report a value in a histogram
    pub fn histogram<S: AsRef<str>>(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_histogram(metric.as_ref(), value.as_ref(), tags);
        }
    }

    /// Report a value in a distribution
    pub fn distribution<S: AsRef<str>>(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_distribution(metric.as_ref(), value.as_ref(), tags);
        }
    }

    /// Report a value in a set
    pub fn set<S: AsRef<str>>(metric: impl AsRef<str>, value: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_set(metric.as_ref(), value.as_ref(), tags);
        }
    }

    /// Report the status of a service
    pub fn service_check<S: AsRef<str>>(
        metric: impl AsRef<str>,
        value: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    ) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_service_check(metric.as_ref(), value, tags, options);
        }
    }

    /// Send a custom event as a title and a body
    pub fn event<S: AsRef<str>>(metric: impl AsRef<str>, text: impl AsRef<str>, tags: impl TagsProvider<S>) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_event(metric.as_ref(), text.as_ref(), tags);
        }
    }

    /// Send a custom event as a title, a body and some options
    pub fn event_with_options<S: AsRef<str>>(
        metric: impl AsRef<str>,
        text: impl AsRef<str>,
        tags: impl TagsProvider<S>,
        options: Option<EventOptions>,
    ) {
        if let Some(instance) = INSTANCE.get() {
            instance.do_event_with_options(metric.as_ref(), text.as_ref(), tags, options);
        }
    }

    /// Acquire a timing guard.
    /// When this guard is dropped, it will emit a timing metric for the duration it
    /// existed. The metric name is metric, and the tags are tags.
    pub fn enter_timing<S: AsRef<str>, P: TagsProvider<S>>(
        metric: impl AsRef<str>,
        tags: P,
    ) -> timing_guard::TimingGuard<S, P> {
        timing_guard::TimingGuard::new(metric, tags)
    }
}

impl<C: DogstatsdClient> Datadog<C> {
    fn new(client: C, tracker_config: TagTrackerConfiguration) -> Self {
        Self {
            inner: client,
            tag_tracker: tracker_config.build(),
        }
    }

    pub(crate) fn do_incr<S: AsRef<str>>(&self, metric: impl AsRef<str>, tags: impl TagsProvider<S>) {
        self.inner.incr(
            metric.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_decr<S: AsRef<str>>(&self, metric: impl AsRef<str>, tags: impl TagsProvider<S>) {
        self.inner.decr(
            metric.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_count<S: AsRef<str>>(&self, metric: impl AsRef<str>, count: i64, tags: impl TagsProvider<S>) {
        self.inner.count(
            metric.as_ref(),
            count,
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_time<S, F, O>(&self, metric: impl AsRef<str>, tags: impl TagsProvider<S>, block: F) -> O
    where
        S: AsRef<str>,
        F: FnOnce() -> O,
    {
        self.inner.time(
            metric.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
            block,
        )
    }

    pub(crate) async fn do_async_time<S, F, T, O>(
        &self,
        metric: &str,
        tags: impl TagsProvider<S> + Send + Sync,
        block: F,
    ) -> O
    where
        S: AsRef<str> + Sync,
        F: FnOnce() -> T + Send,
        T: Future<Output = O> + Send,
    {
        self.inner
            .async_time(
                metric.as_ref(),
                self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
                block,
            )
            .await
    }

    pub(crate) fn do_timing<S: AsRef<str>>(&self, metric: impl AsRef<str>, ms: i64, tags: impl TagsProvider<S>) {
        self.inner.timing(
            metric.as_ref(),
            ms,
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_gauge<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl TagsProvider<S>,
    ) {
        self.inner.gauge(
            metric.as_ref(),
            value.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_histogram<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl TagsProvider<S>,
    ) {
        self.inner.histogram(
            metric.as_ref(),
            value.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_distribution<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl TagsProvider<S>,
    ) {
        self.inner.distribution(
            metric.as_ref(),
            value.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_set<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        value: impl AsRef<str>,
        tags: impl TagsProvider<S>,
    ) {
        self.inner.set(
            metric.as_ref(),
            value.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_service_check<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        value: ServiceStatus,
        tags: impl TagsProvider<S>,
        options: Option<ServiceCheckOptions>,
    ) {
        self.inner.service_check(
            metric.as_ref(),
            value,
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
            options,
        );
    }

    pub(crate) fn do_event<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        text: impl AsRef<str>,
        tags: impl TagsProvider<S>,
    ) {
        self.inner.event(
            metric.as_ref(),
            text.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
        );
    }

    pub(crate) fn do_event_with_options<S: AsRef<str>>(
        &self,
        metric: impl AsRef<str>,
        text: impl AsRef<str>,
        tags: impl TagsProvider<S>,
        options: Option<EventOptions>,
    ) {
        self.inner.event_with_options(
            metric.as_ref(),
            text.as_ref(),
            self.tag_tracker.track(&self.inner, metric.as_ref(), tags),
            options,
        );
    }
}
