//! Error module for this crate

use dogstatsd::DogstatsdError;
use thiserror::Error;

/// the main Error type for the crate
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unable to initialize Datadog global service with once_cell, because the service was already initialized. You should call Datadog::init() just once.")]
    OnceCellAlreadyInitialized,
    #[error(
        "Unable to initialize environment type. The accepted values are 'local', 'qa', 'staging' and 'production'"
    )]
    WrongEnvironmentDefinition,
    #[error(transparent)]
    DogstatsdError(#[from] DogstatsdError),
}

#[cfg(test)]
impl Error {
    pub fn is_once_cell_already_initialized(&self) -> bool {
        match self {
            Error::OnceCellAlreadyInitialized => true,
            Error::WrongEnvironmentDefinition => false,
            Error::DogstatsdError(_) => false,
        }
    }
}
