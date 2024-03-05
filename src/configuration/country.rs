use std::{fmt::Display, str::FromStr};

use crate::error::Error;

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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "common" => Ok(Self::Common),
            "it" => Ok(Self::It),
            "es" => Ok(Self::Es),
            "uk" => Ok(Self::Uk),
            _ => Err(Error::WrongCountryDefinition),
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
    use crate::configuration::Configuration;

    use super::*;

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
}
