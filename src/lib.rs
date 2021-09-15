use crate::configuration::Configuration;

use crate::client::DogstatsdClient;
use crate::error::Error;
use dogstatsd::Client;
use once_cell::sync::OnceCell;

mod adapter;
pub mod client;
pub mod configuration;
pub mod error;

static INSTANCE: OnceCell<Datadog> = OnceCell::new();

pub struct Datadog {
    client: Box<dyn DogstatsdClient + Send + Sync>,
    /// tells if metric should be reported. If false, nothing is sent to the udp socket.
    is_reporting_enabled: bool,
}

impl Datadog {
    pub fn init(configuration: impl Configuration) -> Result<(), Error> {
        let dogstatsd_client_options = dogstatsd::Options::new(
            configuration.from_addr(),
            configuration.to_addr(),
            configuration.namespace(),
        );
        INSTANCE
            .set(Self {
                client: Box::new(Client::new(dogstatsd_client_options)?),
                is_reporting_enabled: configuration.is_reporting_enabled(),
            })
            .map_err(|_| Error::OnceCell)?;

        Ok(())
    }

    pub fn global() -> &'static Datadog {
        INSTANCE.get().expect("logger is not initialized")
    }
}

#[cfg(test)]
impl Datadog {
    pub fn init_test(
        mock: impl DogstatsdClient + 'static + Send + Sync,
        is_reporting_enabled: bool,
    ) -> Result<(), Error> {
        INSTANCE
            .set(Self {
                client: Box::new(mock),
                is_reporting_enabled,
            })
            .map_err(|_| Error::OnceCell)?;
        Ok(())
    }
}

impl DogstatsdClient for Datadog {
    fn incr<'a>(&self, stat: &'a str, tags: Vec<&'a str>) {
        if self.is_reporting_enabled {
            self.client.incr(stat, tags);
        }
    }
}

#[macro_export(local_inner_macros)]
macro_rules! incr {
    ($stat:tt) => {
        $crate::Datadog::global().incr($stat.as_ref(), std::vec::Vec::new())
    };
    ($stat:tt; $( $key:expr => $value:expr ), *) => {
        {
            let tags = std::vec![
                $(
                    std::format!("{}:{}", $key, $value)
                ), *
            ];
            $crate::Datadog::global().incr($stat.as_ref(), tags.into_iter().to_vec())
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn my_test() {
        let mut mock = crate::client::MockDogstatsdClient::new();
        mock.expect_incr().once().return_const(());
        Datadog::init_test(mock, true).unwrap();

        incr!("test");
    }
}
