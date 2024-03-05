//! Configuration module

use crate::error::Error as PrimaDatadogError;
use crate::TagTrackerConfiguration;
use std::fmt::Display;
use std::str::FromStr;

/// By binding to 0.0.0.0:0 we're just letting the OS assign us a port, and letting anyone send us UDP packets on that port
/// This is fine since the statsd protocol is unidirectional(only sends data) to the server
/// We can't just bind to a different address(eg. localhost) because that would prevent us from sending data to the server
const DEFAULT_FROM_ADDR: &str = "0.0.0.0:0";

/// The struct that represents options for the Datadog client in Prima.
pub struct Configuration {
    to_addr: String,
    from_addr: String,
    namespace: String,
    tags: Vec<String>,
    tracker: TagTrackerConfiguration,
    socket_path: Option<String>,
    batching_options: Option<dogstatsd::BatchingOptions>,
}

impl Configuration {
    pub fn new(to_addr: &str, namespace: &str) -> Self {
        Self {
            to_addr: to_addr.to_string(),
            from_addr: DEFAULT_FROM_ADDR.to_string(),
            namespace: namespace.to_string(),
            tags: vec![],
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
        self.with_tag("prima:country", &country)
    }

    pub fn with_tracker_configuration(mut self, tracker: TagTrackerConfiguration) -> Self {
        self.tracker = tracker;
        self
    }

    pub fn with_socket_path(mut self, socket_path: String) -> Self {
        self.socket_path = Some(socket_path);
        self
    }

    pub fn with_batching_options(mut self, batching_options: dogstatsd::BatchingOptions) -> Self {
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

    pub fn batching_options(&self) -> Option<dogstatsd::BatchingOptions> {
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

/// Represent an environment in which the datadog client runs.
/// This is useful for enforcing rules based on environment for every application that uses the library.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Environment {
    Dev,
    Staging,
    Production,
}

impl Environment {
    /// Returns the string representation of the environment.
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Dev => "dev",
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
    Common,
    It,
    Es,
    Uk,
}

impl Country {
    /// Returns the string representation of the country.
    pub fn as_str(&self) -> &'static str {
        match self {
            Country::Common => "common",
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
            "common" => Ok(Self::Common),
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
        let config = Configuration::new("to_addr", "namespace")
            .with_from_addr("from_addr")
            .with_tag("key", &"value")
            .with_tag("count", &count);

        assert_eq!(config.default_tags(), vec!["key:value", "count:1"]);
    }

    #[test]
    pub fn test_environment() {
        let config = Configuration::new("to_addr", "namespace")
            .with_from_addr("from_addr")
            .with_environment(Environment::Dev);

        assert_eq!(config.default_tags(), vec!["env:dev"]);
    }

    #[test]
    pub fn test_country() {
        let config = Configuration::new("to_addr", "namespace")
            .with_from_addr("from_addr")
            .with_country(Country::It);

        assert_eq!(config.default_tags(), vec!["prima:country:it"]);

        let config = Configuration::new("to_addr", "namespace")
            .with_from_addr("from_addr")
            .with_country(Country::It)
            .with_country(Country::Es);

        // Datadog tag keys are allowed to map to multiple values, and I suppose we're ok with that too (e.g. cross-country infra down the line?)
        assert_eq!(config.default_tags(), vec!["prima:country:it", "prima:country:es"]);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_country_serde() {
        for country in [Country::Common, Country::It, Country::Es, Country::Uk] {
            let serialized = serde_json::to_string(&country).unwrap();
            let deserialized: Country = serde_json::from_str(&serialized).unwrap();
            assert_eq!(country, deserialized);
        }
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_environment_serde() {
        for environment in [Environment::Production, Environment::Staging, Environment::Dev] {
            let serialized = serde_json::to_string(&environment).unwrap();
            let deserialized: Environment = serde_json::from_str(&serialized).unwrap();
            assert_eq!(environment, deserialized);
        }
    }
}
