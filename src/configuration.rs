use crate::error::Error;
use crate::error::PrimaDatadogResult;
use std::str::FromStr;

/// The struct that represents the options available for the Datadog client.
pub struct Configuration {
    /// The address of the udp socket we'll send metrics and events to.
    to_addr: String,
    /// The address of the udp socket we'll bind to for sending.
    from_addr: String,
    /// A namespace to prefix all metrics with, joined with a '.'.
    namespace: Option<String>,
    /// when false, no metric gets reported. Nothing is sent to the udp socket.
    reporting_enabled: bool,
}

impl Configuration {
    /// create a configuration with good default.
    /// You should pass something that can be converted to an `Environment` type.
    pub fn from_env(to_addr: &str, env: &str) -> PrimaDatadogResult<Self> {
        let environment = env.parse()?;
        let reporting_enabled = match environment {
            Environment::Qa => false,
            _ => true,
        };

        Ok(Self {
            to_addr: to_addr.to_string(),
            from_addr: "0.0.0.0:0".to_string(),
            namespace: None,
            reporting_enabled,
        })
    }

    /// set to_addr
    pub fn set_to_addr(&mut self, to_addr: &str) {
        self.to_addr = to_addr.to_string();
    }

    /// set from_addr
    pub fn set_from_addr(&mut self, from_addr: String) {
        self.from_addr = from_addr;
    }

    /// set namespace
    pub fn set_namespace(&mut self, namespace: String) {
        self.namespace = Some(namespace);
    }

    #[test]
    pub fn for_test() -> Self {
        Self {
            to_addr: "0.0.0.0:0".to_string(),
            from_addr: "0.0.0.0:0".to_string(),
            namespace: Some("test".to_string()),
            reporting_enabled: true,
        }
    }
}

impl From<&Configuration> for dogstatsd::Options {
    fn from(configuration: &Configuration) -> Self {
        dogstatsd::Options::new(
            configuration.from_addr.as_str(),
            configuration.to_addr.as_str(),
            configuration.namespace.as_ref().unwrap_or(&"".to_string()),
        )
    }
}

/// Represent an environment in which the datadog client is running.
/// This is useful for enforcing rules based on environment for every application that uses the library.
pub enum Environment {
    Local,
    Qa,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = Error;

    fn from_str(s: &str) -> PrimaDatadogResult<Self> {
        match s {
            "local" => Ok(Self::Local),
            "qa" => Ok(Self::Qa),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(Error::WrongEnvironmentDefinition),
        }
    }
}
