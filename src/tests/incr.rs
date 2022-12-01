use crate::incr;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn incr_with_literal() {
    let mock = mocks::incr_mock("test", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_incr("test", EMPTY_TAGS);
}

#[test]
pub fn incr_with_type() {
    let mock = mocks::incr_mock("test1_event", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_incr(TestEvent::Test1, EMPTY_TAGS);
}

#[test]
pub fn incr_with_literal_and_tags() {
    let mock = mocks::incr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_incr("test", vec!["added:tag".to_string()]);
}

#[test]
pub fn incr_with_type_and_tags() {
    let mock = mocks::incr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_incr(TestEvent::Test1, vec!["added:tag".to_string()]);
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    incr!("test");
    // just literal tags
    incr!("test"; "literal" => 1);
    // just expression tags
    incr!("test"; "expression" => tag);
    // mixed tags
    incr!("test"; "literal" => 1, "expression" => tag);
}
