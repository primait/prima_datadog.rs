//! Configuration module

mod prima;
mod test;

pub use prima::{Environment, PrimaConfiguration};
pub use test::TestConfiguration;

/// A trait representing a valid configuration entity
#[allow(clippy::wrong_self_convention)]
pub trait Configuration {
    /// The address of the udp socket we'll bind to for sending
    fn to_addr(&self) -> &str;
    /// The address of the udp socket we'll send metrics and events to
    fn from_addr(&self) -> &str;
    /// A namespace to prefix all metrics with, joined with a '.'.
    fn namespace(&self) -> &str;
    /// Whether to send metrics or not.
    /// This is useful to make the client silent in certain condition
    fn is_reporting_enabled(&self) -> bool;
    /// Default tags to be sent with every metric reporting
    fn default_tags(&self) -> Vec<String>;
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

    fn is_reporting_enabled(&self) -> bool {
        true
    }

    fn default_tags(&self) -> Vec<String> {
        vec![]
    }
}
