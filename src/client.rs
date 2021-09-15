use crate::error::Error;

#[cfg(test)]
use mockall::{automock, predicate::*};

/// a Dogstatsd client trait
#[cfg_attr(test, automock)]
pub trait DogstatsdClient {
    fn incr<'a>(&self, stat: &'a str, tags: Vec<&'a str>);
}
