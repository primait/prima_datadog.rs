//! this is the configuration module for prima environment

use crate::configuration::Configuration;
use crate::error::Error as PrimaDatadogError;
use std::convert::TryInto;
use std::str::FromStr;

/// The struct that represents options for the Datadog client in Prima.
pub struct PrimaConfiguration {
    to_addr: String,
    from_addr: String,
    namespace: String,
    environment: Environment,
}

impl PrimaConfiguration {
    pub fn new(
        to_addr: &str,
        from_addr: &str,
        namespace: &str,
        environment: impl TryInto<Environment, Error = PrimaDatadogError>,
    ) -> Result<Self, PrimaDatadogError> {
        Ok(Self {
            to_addr: to_addr.to_string(),
            from_addr: from_addr.to_string(),
            namespace: namespace.to_string(),
            environment: environment.try_into()?,
        })
    }
}

impl Configuration for PrimaConfiguration {
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
        match self.environment {
            Environment::Qa => false,
            _ => true,
        }
    }

    fn default_tags(&self) -> Vec<String> {
        vec![format!("env:{}", self.environment.to_string())]
    }
}

/// Represent an environment in which the datadog client is running.
/// This is useful for enforcing rules based on environment for every application that uses the library.
pub enum Environment {
    Dev,
    Qa,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = PrimaDatadogError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "qa" => Ok(Self::Qa),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(PrimaDatadogError::WrongEnvironmentDefinition),
        }
    }
}

impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            Environment::Dev => "dev".to_string(),
            Environment::Qa => "qa".to_string(),
            Environment::Staging => "staging".to_string(),
            Environment::Production => "production".to_string(),
        }
    }
}
