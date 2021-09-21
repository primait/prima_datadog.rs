mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn gauge_with_literal() {
    let mock = mocks::gauge_mock("test", "test_value", &[]);
    Datadog::new(mock, true, vec![]).gauge("test", "test_value", vec![]);
}

#[test]
pub fn gauge_with_type() {
    let mock = mocks::gauge_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, vec![]).gauge(common::TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn gauge_with_literal_and_tags() {
    let mock = mocks::gauge_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).gauge(
        "test",
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn gauge_with_type_and_tags() {
    let mock = mocks::gauge_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).gauge(
        common::TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}
