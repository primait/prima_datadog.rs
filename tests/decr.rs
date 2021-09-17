mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn decr_with_literal() {
    let mock = mocks::decr_mock("env:test", &[]);
    Datadog::new(mock, true, vec![]).decr("env:test", vec![]);
}

#[test]
pub fn decr_with_type() {
    let mock = mocks::decr_mock("test1_event", &[]);
    Datadog::new(mock, true, vec![]).decr(common::TestEvent::Test1, vec![]);
}

#[test]
pub fn decr_with_literal_and_tags() {
    let mock = mocks::decr_mock("env:test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).decr("env:test", vec!["added:tag".to_string()]);
}

#[test]
pub fn decr_with_type_and_tags() {
    let mock = mocks::decr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"])
        .decr(common::TestEvent::Test1, vec!["added:tag".to_string()]);
}
