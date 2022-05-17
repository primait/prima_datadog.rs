mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn decr_with_literal() {
    let mock = mocks::decr_mock("test", &[]);
    Datadog::new(mock).decr("test", vec![]);
}

#[test]
pub fn decr_with_type() {
    let mock = mocks::decr_mock("test1_event", &[]);
    Datadog::new(mock).decr(common::TestEvent::Test1, vec![]);
}

#[test]
pub fn decr_with_literal_and_tags() {
    let mock = mocks::decr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock).decr("test", vec!["added:tag".to_string()]);
}

#[test]
pub fn decr_with_type_and_tags() {
    let mock = mocks::decr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock).decr(common::TestEvent::Test1, vec!["added:tag".to_string()]);
}
