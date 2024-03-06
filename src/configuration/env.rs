use std::str::FromStr;

use crate::error::Error;

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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(Error::WrongEnvironmentDefinition),
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
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

#[cfg(test)]
mod tests {
    use crate::configuration::Configuration;

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
    pub fn test_environment() {
        let config = Configuration::new("to_addr", "namespace")
            .with_from_addr("from_addr")
            .with_environment(Environment::Dev);

        assert_eq!(config.default_tags(), vec!["env:dev"]);
    }
}
