use crate::count;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn count_with_literal() {
    let mock = mocks::count_mock("test", 10, &[]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_count("test", 10, EMPTY_TAGS);
}

#[test]
pub fn count_with_type() {
    let mock = mocks::count_mock("test1_event", 10, &[]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_count(TestEvent::Test1, 10, EMPTY_TAGS);
}

#[test]
pub fn count_with_literal_and_tags() {
    let mock = mocks::count_mock("test", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_count("test", 10, vec!["added:tag".to_string()]);
}

#[test]
pub fn count_with_type_and_tags() {
    let mock = mocks::count_mock("test1_event", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, TagTrackerConfiguration::new()).do_count(TestEvent::Test1, 10, vec!["added:tag".to_string()]);
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    count!("test", 10);
    // just literal tags
    count!("test", 10; "tag1" => "value1", "tag2" => "value2");
    // just expression tags
    count!("test", 10; tag => "value");
    // mixed tags
    count!("test", 1; "literal" => 1, "expression" => tag);
}
