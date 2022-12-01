use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::time;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn time_with_literal() {
    let mock = mocks::time_mock("test", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_time("test", EMPTY_TAGS, || {});
}

#[test]
pub fn time_with_type() {
    let mock = mocks::time_mock("test1_event", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_time(TestEvent::Test1, EMPTY_TAGS, || {});
}

#[test]
pub fn time_with_literal_and_tags() {
    let mock = mocks::time_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_time("test", vec!["added:tag".to_string()], || {});
}

#[test]
pub fn time_with_type_and_tags() {
    let mock = mocks::time_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_time(
        TestEvent::Test1,
        vec!["added:tag".to_string()],
        || {},
    );
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    time!("test", || {});
    // just literal tags
    time!("test", || {}; "literal" => 1);
    // just expression tags
    time!("test", || {}; "expression" => tag);
    // mixed tags
    time!("test", || {}; "literal" => 1, "expression" => tag);
}
