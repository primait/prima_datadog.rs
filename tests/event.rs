mod common;
mod mocks;

use prima_datadog::Datadog;

#[test]
pub fn event_with_literal() {
    let mock = mocks::event_mock("test", "test_value", &[]);
    Datadog::new(mock, true, vec![]).event("test", "test_value", vec![]);
}

#[test]
pub fn event_with_type() {
    let mock = mocks::event_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, vec![]).event(common::TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn event_with_ident() {
    let text = "test_value";
    let mock = mocks::event_mock("event", "test_value", &[]);
    Datadog::new(mock, true, vec![]).event("event", text, vec![]);
}

#[test]
pub fn event_with_literal_and_tags() {
    let mock = mocks::event_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).event(
        "test",
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn event_with_type_and_tags() {
    let mock = mocks::event_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, ["env:test"]).event(
        common::TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}
