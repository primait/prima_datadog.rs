use crate::set;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn set_with_literal() {
    let mock = mocks::set_mock("test", "test_value", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_set("test", "test_value", EMPTY_TAGS);
}

#[test]
pub fn set_with_type() {
    let mock = mocks::set_mock("test1_event", "test_value", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_set(TestEvent::Test1, "test_value", EMPTY_TAGS);
}

#[test]
pub fn set_with_literal_and_tags() {
    let mock = mocks::set_mock("test", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_set(
        "test",
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn set_with_type_and_tags() {
    let mock = mocks::set_mock("test1_event", "test_value", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_set(
        TestEvent::Test1,
        "test_value",
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    set!("test", "test_value");
    // just literal tags
    set!("test", "test_value"; "literal" => 1);
    // just expression tags
    set!("test", "test_value"; "expression" => tag);
    // mixed tags
    set!("test", "test_value"; "literal" => 1, "expression" => tag);
}
