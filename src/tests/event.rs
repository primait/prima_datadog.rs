use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::DatadogWrapper;
use crate::EMPTY_TAGS;

#[test]
pub fn event_with_literal() {
    let mock = mocks::event_mock("test", "test_value", EMPTY_TAGS);
    DatadogWrapper::new(mock, true).do_event("test", "test_value", EMPTY_TAGS);
}

#[test]
pub fn event_with_type() {
    let mock = mocks::event_mock("test1_event", "test_value", EMPTY_TAGS);
    DatadogWrapper::new(mock, true).do_event(TestEvent::Test1, "test_value", EMPTY_TAGS);
}

#[test]
pub fn event_with_literal_and_tags() {
    let mock = mocks::event_mock("test", "test_value", &["added:tag", "env:test"]);
    DatadogWrapper::new(mock, true).do_event("test", "test_value", &["added:tag"]);
}

#[test]
pub fn event_with_type_and_tags() {
    let mock = mocks::event_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    DatadogWrapper::new(mock, true).do_event(TestEvent::Test1, "test_value", &["added:tag"]);
}
