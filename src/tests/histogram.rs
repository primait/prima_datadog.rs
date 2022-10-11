use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::DEFAULT_TAG_THRESHOLD;

#[test]
pub fn histogram_with_literal() {
    let mock = mocks::histogram_mock("test", "test_value", &[]);
    Datadog::new(mock, true, DEFAULT_TAG_THRESHOLD).do_histogram("test", "test_value", vec![]);
}

#[test]
pub fn histogram_with_type() {
    let mock = mocks::histogram_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, DEFAULT_TAG_THRESHOLD).do_histogram(TestEvent::Test1, "test_value", vec![]);
}

#[test]
pub fn histogram_with_literal_and_tags() {
    let mock = mocks::histogram_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, DEFAULT_TAG_THRESHOLD).do_histogram("test", "test_value", vec!["added:tag".to_string()]);
}

#[test]
pub fn histogram_with_type_and_tags() {
    let mock = mocks::histogram_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, DEFAULT_TAG_THRESHOLD).do_histogram(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}
