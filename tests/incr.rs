mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn incr_with_literal() {
    let mock = mocks::incr_mock("env:test", &[]);
    Datadog::new(mock, true, vec![]).incr("env:test", vec![]);
}

#[test]
pub fn incr_with_type() {
    let mock = mocks::incr_mock("test1_event", &[]);
    Datadog::new(mock, true, vec![]).incr(common::TestEvent::Test1, vec![]);
}

#[test]
pub fn incr_with_literal_and_tags() {
    let mock = mocks::incr_mock("env:test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).incr("env:test", vec!["added:tag".to_string()]);
}

#[test]
pub fn incr_with_type_and_tags() {
    let mock = mocks::incr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"])
        .incr(common::TestEvent::Test1, vec!["added:tag".to_string()]);
}
