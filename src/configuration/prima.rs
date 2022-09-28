//! Configuration module for prima environment

use crate::configuration::Configuration;
use crate::error::Error as PrimaDatadogError;
use std::str::FromStr;

/// The struct that represents options for the Datadog client in Prima.
pub struct PrimaConfiguration {
    to_addr: String,
    from_addr: String,
    namespace: String,
    environment: Environment,
    tags: Vec<String>,
}

impl PrimaConfiguration {
    pub fn new(to_addr: &str, from_addr: &str, namespace: &str, environment: Environment) -> Self {
        let env_str = environment.to_string();
        Self {
            to_addr: to_addr.to_string(),
            from_addr: from_addr.to_string(),
            namespace: namespace.to_string(),
            environment,
            tags: vec![format!("env:{}", env_str)],
        }
    }

    pub fn with_tag<T: ToString>(mut self, key: &str, value: &T) -> Self {
        self.tags.push(format!("{}:{}", key, value.to_string()));
        self
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
        self.environment != Environment::Qa
    }

    fn default_tags(&self) -> Vec<String> {
        self.tags.clone()
    }
}

/// Represent an environment in which the datadog client runs.
/// This is useful for enforcing rules based on environment for every application that uses the library.
#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_str() {
        assert_eq!(Some(Environment::Dev), "dev".parse().ok());
        assert_eq!(Some(Environment::Qa), "qa".parse().ok());
        assert_eq!(Some(Environment::Staging), "staging".parse().ok());
        assert_eq!(Some(Environment::Production), "production".parse().ok());
    }

    #[test]
    pub fn test_from_str_err() {
        //assert_eq!(None, "".parse::<Environment>().err());
        assert_eq!(None, "whatever".parse::<Environment>().ok());
    }

    #[test]
    pub fn test_tags() {
        let count = 1;
        let config = PrimaConfiguration::new("to_addr", "from_addr", "namespace", Environment::Dev)
            .with_tag("key", &"value")
            .with_tag("count", &count);

        assert_eq!(config.default_tags(), vec!["env:dev", "key:value", "count:1"]);
    }
}
