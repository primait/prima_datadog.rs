use crate::configuration::Configuration;

/// The struct that represents options for the Datadog client in a test configuration.
/// It's a NoOp configuration
#[derive(Default)]
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

    fn default_tags(&self) -> Vec<String> {
        vec![]
    }

    fn socket_path(&self) -> Option<String> {
        None
    }

    fn batching_options(&self) -> Option<dogstatsd::BatchingOptions> {
        None
    }
}
