use dogstatsd::DogstatsdError;
use thiserror::Error;

pub type PrimaDatadogResult<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    DogstatsdError(#[from] DogstatsdError),
    #[error("Unable to initialize environment type. The accepted values are 'local', 'qa', 'staging' and 'production'")]
    WrongEnvironmentDefinition,
}
