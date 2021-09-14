use crate::configuration::Configuration;
use crate::error::PrimaDatadogResult;
use dogstatsd::Client;

mod configuration;
pub mod error;

//use once_cell::sync::Lazy;

// static DATADOG: Lazy<Datadog> = Lazy::new(|| {
//     let env = std::env::var("APP_ENV").expect("APP_ENV env variable must be set");
//
//     let host = std::env::var("DATADOG_HOST").expect("DATADOG_HOST env variable must be set");
//
//     Datadog::new(env.as_str(), host.as_str())
// });

struct Datadog {
    client: dogstatsd::Client,
    configuration: Configuration,
}

impl Datadog {
    pub fn new(configuration: Configuration) -> PrimaDatadogResult<Self> {
        Ok(Self {
            client: Client::new((&configuration).into())?,
            configuration,
        })
    }
}
