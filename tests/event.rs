mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn event_with_literal() {
    let mock = mocks::event_mock("test", "test_value", &[]);
    Datadog::new(mock).event("test", "test_value", vec![]);
}

#[test]
pub fn event_with_type() {
    let mock = mocks::event_mock("test1_event", "test_value", &[]);
    Datadog::new(mock).event(common::TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn event_with_literal_and_tags() {
    let mock = mocks::event_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock).event("test", "test_value", vec!["added:tag".to_string()]);
}

#[test]
pub fn event_with_type_and_tags() {
    let mock = mocks::event_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock).event(common::TestEvent::Test1, "test_value", vec!["added:tag".to_string()]);
}
