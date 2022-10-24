use crate::configuration::{Environment, PrimaConfiguration};

use super::*;

mod count;
mod decr;
mod distribution;
mod event;
mod gauge;
mod histogram;
mod incr;
mod mocks;
mod service_check;
mod set;
mod tag_check;
mod time;
mod timing;
mod tracker;

#[test]
pub fn double_initialization() {
    let datadog = Datadog::init(PrimaConfiguration::new(
        "10.1.2.3:8125",
        "127.0.0.1:9000",
        "",
        Environment::Dev,
    ));
    assert!(datadog.is_ok());
    let datadog2 = Datadog::init(PrimaConfiguration::new(
        "10.1.2.3:8125",
        "127.0.0.1:9000",
        "",
        Environment::Production,
    ));
    assert!(datadog2.err().unwrap().is_once_cell_already_initialized());
}

pub enum TestEvent {
    Test1,
}

impl AsRef<str> for TestEvent {
    fn as_ref(&self) -> &str {
        match self {
            TestEvent::Test1 => "test1_event",
        }
    }
}

impl std::fmt::Display for TestEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
