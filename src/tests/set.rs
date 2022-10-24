use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TrackerConfiguration;

#[test]
pub fn set_with_literal() {
    let mock = mocks::set_mock("test", "test_value", &[]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_set("test", "test_value", vec![]);
}

#[test]
pub fn set_with_type() {
    let mock = mocks::set_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_set(TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn set_with_literal_and_tags() {
    let mock = mocks::set_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_set("test", "test_value", vec!["added:tag".to_string()]);
}

#[test]
pub fn set_with_type_and_tags() {
    let mock = mocks::set_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_set(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}
