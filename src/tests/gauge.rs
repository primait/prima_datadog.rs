use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;

#[test]
pub fn gauge_with_literal() {
    let mock = mocks::gauge_mock("test", "test_value", &[]);
    Datadog::new(mock, true)._gauge("test", "test_value", vec![]);
}

#[test]
pub fn gauge_with_type() {
    let mock = mocks::gauge_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true)._gauge(TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn gauge_with_literal_and_tags() {
    let mock = mocks::gauge_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true)._gauge("test", "test_value", vec!["added:tag".to_string()]);
}

#[test]
pub fn gauge_with_type_and_tags() {
    let mock = mocks::gauge_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true)._gauge(TestEvent::Test1, "test_value", vec!["added:tag".to_string()]);
}
