//! prima_datadog errors

use dogstatsd::DogstatsdError;
use thiserror::Error;

/// the main Error type for the crate
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to initialize Datadog global service with once_cell")]
    OnceCell,
    #[error("Unable to initialize environment type. The accepted values are 'local', 'qa', 'staging' and 'production'")]
    WrongEnvironmentDefinition,
    #[error(transparent)]
    DogstatsdError(#[from] DogstatsdError),
}
