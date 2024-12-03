//! Configuration module

mod country;
mod env;

pub use country::Country;
pub use env::Environment;

pub use dogstatsd::BatchingOptions;

use crate::TagTrackerConfiguration;
use std::fmt::Display;

/// By binding to 0.0.0.0:0 we're just letting the OS assign us a port, and letting anyone send us UDP packets on that port
///
/// This is fine since the statsd protocol is unidirectional (only sends data) to the server.
/// We can't just bind to a different address (e.g. localhost) because that would prevent us from sending data to the server
const DEFAULT_FROM_ADDR: &str = "0.0.0.0:0";

/// The struct that represents options for the Datadog client in Prima.
pub struct Configuration {
    to_addr: String,
    from_addr: String,
    namespace: String,
    tags: Vec<String>,
    tracker: TagTrackerConfiguration,
    socket_path: Option<String>,
    batching_options: Option<BatchingOptions>,
}

impl Configuration {
    pub fn new(to_addr: &str, namespace: &str) -> Self {
        Self {
            to_addr: to_addr.to_string(),
            from_addr: DEFAULT_FROM_ADDR.to_string(),
            namespace: namespace.to_string(),
            tags: get_env_tags(),
            tracker: TagTrackerConfiguration::new(),
            socket_path: None,
            batching_options: None,
        }
    }

    pub fn with_from_addr(mut self, from_addr: &str) -> Self {
        self.from_addr = from_addr.to_string();
        self
    }

    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.tags.push(format!("env:{}", environment));
        self
    }

    pub fn with_tag<T: Display>(mut self, key: &str, value: &T) -> Self {
        self.tags.push(format!("{}:{}", key, value));
        self
    }

    pub fn with_country(self, country: Country) -> Self {
        self.with_tag("prima_country", &country)
    }

    pub fn with_tracker_configuration(mut self, tracker: TagTrackerConfiguration) -> Self {
        self.tracker = tracker;
        self
    }

    pub fn with_socket_path(mut self, socket_path: String) -> Self {
        self.socket_path = Some(socket_path);
        self
    }

    pub fn with_batching_options(mut self, batching_options: BatchingOptions) -> Self {
        self.batching_options = Some(batching_options);
        self
    }

    pub fn to_addr(&self) -> &str {
        self.to_addr.as_str()
    }

    pub fn from_addr(&self) -> &str {
        self.from_addr.as_str()
    }

    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    pub fn default_tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn socket_path(&self) -> Option<String> {
        self.socket_path.clone()
    }

    pub fn batching_options(&self) -> Option<BatchingOptions> {
        self.batching_options
    }

    pub fn take_tracker_config(&mut self) -> TagTrackerConfiguration {
        std::mem::replace(&mut self.tracker, TagTrackerConfiguration::new())
    }
}

impl From<Configuration> for dogstatsd::Options {
    fn from(value: Configuration) -> Self {
        dogstatsd::Options::new(
            value.from_addr(),
            value.to_addr(),
            value.namespace(),
            value.default_tags(),
            value.socket_path(),
            value.batching_options(),
        )
    }
}

fn get_env_tags() -> Vec<String> {
    let mut tags = vec![];
    if let Ok(part_of) = std::env::var("KUBE_APP_PART_OF") {
        tags.push(format!("kube_app_part_of:{}", part_of));
    }
    if let Ok(managed_by) = std::env::var("KUBE_APP_MANAGED_BY") {
        tags.push(format!("kube_app_managed_by:{}", managed_by));
    }
    if let Ok(version) = std::env::var("KUBE_APP_VERSION") {
        tags.push(format!("kube_app_version:{}", version));
    }
    if let Ok(instance) = std::env::var("KUBE_APP_INSTANCE") {
        tags.push(format!("kube_app_instance:{}", instance));
    }
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_tags() {
        let count = 1;
        let config = Configuration::new("to_addr", "namespace")
            .with_from_addr("from_addr")
            .with_tag("key", &"value")
            .with_tag("count", &count);

        assert_eq!(config.default_tags(), vec!["key:value", "count:1"]);
    }
}
