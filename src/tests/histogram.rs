use crate::histogram;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn histogram_with_literal() {
    let mock = mocks::histogram_mock("test", "test_value", &[]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_histogram("test", "test_value", EMPTY_TAGS);
}

#[test]
pub fn histogram_with_type() {
    let mock = mocks::histogram_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_histogram(TestEvent::Test1, "test_value", EMPTY_TAGS);
}

#[test]
pub fn histogram_with_literal_and_tags() {
    let mock = mocks::histogram_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_histogram(
        "test",
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn histogram_with_type_and_tags() {
    let mock = mocks::histogram_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_histogram(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    histogram!("test", "test_value");
    // just literal tags
    histogram!("test", "test_value"; "literal" => 1);
    // just expression tags
    histogram!("test", "test_value"; "expression" => tag);
    // mixed tags
    histogram!("test", "test_value"; "literal" => 1, "expression" => tag);
}
