use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::timing;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn timing_with_literal() {
    let mock = mocks::timing_mock("test", 10, &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_timing("test", 10, EMPTY_TAGS);
}

#[test]
pub fn timing_with_type() {
    let mock = mocks::timing_mock("test1_event", 10, &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_timing(TestEvent::Test1, 10, EMPTY_TAGS);
}

#[test]
pub fn timing_with_literal_and_tags() {
    let mock = mocks::timing_mock("test", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_timing("test", 10, vec!["added:tag".to_string()]);
}

#[test]
pub fn timing_with_type_and_tags() {
    let mock = mocks::timing_mock("test1_event", 10, &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_timing(
        TestEvent::Test1,
        10,
        vec!["added:tag".to_string()],
    );
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    timing!("test", 10);
    // just literal tags
    timing!("test", 10; "literal" => 1);
    // just expression tags
    timing!("test", 10; "expression" => tag);
    // mixed tags
    timing!("test", 1000; "literal" => 1, "expression" => tag);
}
