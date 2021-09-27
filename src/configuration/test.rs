use crate::configuration::Configuration;

/// The struct that represents options for the Datadog client in a test configuration.
/// It's a NoOp configuration
pub struct TestConfiguration;

impl Configuration for TestConfiguration {
    fn to_addr(&self) -> &str {
        "0.0.0.0:0"
    }

    fn from_addr(&self) -> &str {
        "0.0.0.0:0"
    }

    fn namespace(&self) -> &str {
        "test"
    }

    fn is_reporting_enabled(&self) -> bool {
        false
    }

    fn default_tags(&self) -> Vec<String> {
        vec![]
    }
}

impl Default for TestConfiguration {
    fn default() -> Self {
        TestConfiguration {}
    }
}
