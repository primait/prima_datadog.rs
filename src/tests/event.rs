use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn event_with_literal() {
    let mock = mocks::event_mock("test", "test_value", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_event("test", "test_value", EMPTY_TAGS);
}

#[test]
pub fn event_with_type() {
    let mock = mocks::event_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_event(TestEvent::Test1, "test_value", EMPTY_TAGS);
}

#[test]
pub fn event_with_literal_and_tags() {
    let mock = mocks::event_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_event(
        "test",
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn event_with_type_and_tags() {
    let mock = mocks::event_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_event(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}
