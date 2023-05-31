//! Configuration module for prima environment

use crate::configuration::Configuration;
use crate::error::Error as PrimaDatadogError;
use crate::TagTrackerConfiguration;
use std::fmt::Display;
use std::str::FromStr;

/// The struct that represents options for the Datadog client in Prima.
pub struct PrimaConfiguration {
    to_addr: String,
    from_addr: String,
    namespace: String,
    environment: Environment,
    tags: Vec<String>,
    tracker: TagTrackerConfiguration,
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
            tracker: TagTrackerConfiguration::new(),
        }
    }

    pub fn with_tag<T: std::fmt::Display>(mut self, key: &str, value: &T) -> Self {
        self.tags.push(format!("{}:{}", key, value));
        self
    }

    pub fn with_country(self, country: Country) -> Self {
        self.with_tag("prima:country", &country)
    }

    pub fn with_tracker_configuration(mut self, tracker: TagTrackerConfiguration) -> Self {
        self.tracker = tracker;
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

    fn take_tracker_config(&mut self) -> TagTrackerConfiguration {
        std::mem::replace(&mut self.tracker, TagTrackerConfiguration::new())
    }
}

/// Represent an environment in which the datadog client runs.
/// This is useful for enforcing rules based on environment for every application that uses the library.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Environment {
    Dev,
    Qa,
    Staging,
    Production,
}

impl Environment {
    /// Returns the string representation of the environment.
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Qa => "qa",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
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

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg(feature = "serde")]
impl serde::Serialize for Environment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Environment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct EnvironmentVisitor;
        impl<'de> serde::de::Visitor<'de> for EnvironmentVisitor {
            type Value = Environment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a environment tag")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Environment::from_str(v).map_err(|_| E::custom("unknown environment tag"))
            }
        }
        deserializer.deserialize_str(EnvironmentVisitor)
    }
}

/// Represents the country in which the datadog client runs.
/// This is useful for enforcing rules based on country for every application that uses the library.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Country {
    It,
    Es,
    Uk,
}

impl Country {
    /// Returns the string representation of the country.
    pub fn as_str(&self) -> &'static str {
        match self {
            Country::It => "it",
            Country::Es => "es",
            Country::Uk => "uk",
        }
    }
}

impl FromStr for Country {
    type Err = PrimaDatadogError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "it" => Ok(Self::It),
            "es" => Ok(Self::Es),
            "uk" => Ok(Self::Uk),
            _ => Err(PrimaDatadogError::WrongCountryDefinition),
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg(feature = "serde")]
impl serde::Serialize for Country {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Country {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CountryVisitor;
        impl<'de> serde::de::Visitor<'de> for CountryVisitor {
            type Value = Country;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a country tag")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Country::from_str(v).map_err(|_| E::custom("unknown country tag"))
            }
        }
        deserializer.deserialize_str(CountryVisitor)
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

    #[test]
    pub fn test_country() {
        let config =
            PrimaConfiguration::new("to_addr", "from_addr", "namespace", Environment::Dev).with_country(Country::It);

        assert_eq!(config.default_tags(), vec!["env:dev", "prima:country:it"]);

        let config = PrimaConfiguration::new("to_addr", "from_addr", "namespace", Environment::Dev)
            .with_country(Country::It)
            .with_country(Country::Es);

        // Datadog tag keys are allowed to map to multiple values, and I suppose we're ok with that too (e.g. cross-country infra down the line?)
        assert_eq!(
            config.default_tags(),
            vec!["env:dev", "prima:country:it", "prima:country:es"]
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_country_serde() {
        for country in [Country::It, Country::Es, Country::Uk] {
            let serialized = serde_json::to_string(&country).unwrap();
            let deserialized: Country = serde_json::from_str(&serialized).unwrap();
            assert_eq!(country, deserialized);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_environment_serde() {
        for environment in [
            Environment::Production,
            Environment::Staging,
            Environment::Qa,
            Environment::Dev,
        ] {
            let serialized = serde_json::to_string(&environment).unwrap();
            let deserialized: Environment = serde_json::from_str(&serialized).unwrap();
            assert_eq!(environment, deserialized);
        }
    }
}
