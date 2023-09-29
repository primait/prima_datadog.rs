//! Configuration module

mod prima;
mod test;

pub use prima::{Country, Environment, PrimaConfiguration};
pub use test::TestConfiguration;

use crate::TagTrackerConfiguration;

/// A trait representing a valid configuration entity
#[allow(clippy::wrong_self_convention)]
pub trait Configuration {
    /// The address of the udp socket we'll bind to for sending
    fn to_addr(&self) -> &str;
    /// The address of the udp socket we'll send metrics and events to
    fn from_addr(&self) -> &str;
    /// A namespace to prefix all metrics with, joined with a '.'.
    fn namespace(&self) -> &str;
    /// Default tags to be sent with every metric reporting
    fn default_tags(&self) -> Vec<String>;
    /// if defined, will use UDS instead of UDP and will ignore UDP options
    fn socket_path(&self) -> Option<String>;
    /// if defined, will utilize batching for sending metrics
    fn batching_options(&self) -> Option<dogstatsd::BatchingOptions>;
    /// Get the tag tracker configuration, and reset it to default. See [TagTrackerConfiguration]
    fn take_tracker_config(&mut self) -> TagTrackerConfiguration {
        TagTrackerConfiguration::new()
    }
}

impl Configuration for dogstatsd::Options {
    fn to_addr(&self) -> &str {
        self.to_addr.as_str()
    }

    fn from_addr(&self) -> &str {
        self.from_addr.as_str()
    }

    fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    fn default_tags(&self) -> Vec<String> {
        vec![]
    }

    fn socket_path(&self) -> Option<String> {
        self.socket_path.clone()
    }

    fn batching_options(&self) -> Option<dogstatsd::BatchingOptions> {
        self.batching_options
    }
}
