use crate::decr;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;

#[test]
pub fn decr_with_literal() {
    let mock = mocks::decr_mock("test", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_decr("test", EMPTY_TAGS);
}

#[test]
pub fn decr_with_type() {
    let mock = mocks::decr_mock("test1_event", &[]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_decr(TestEvent::Test1, EMPTY_TAGS);
}

#[test]
pub fn decr_with_literal_and_tags() {
    let mock = mocks::decr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_decr("test", vec!["added:tag".to_string()]);
}

#[test]
pub fn decr_with_type_and_tags() {
    let mock = mocks::decr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_decr(TestEvent::Test1, vec!["added:tag".to_string()]);
}

#[test]
pub fn test_macro() {
    let tag = String::from("tag");
    // no tags
    decr!("test");
    // just literal tags
    decr!("test"; "literal" => 1);
    // just expression tags
    decr!("test"; "expression" => tag);
    // mixed tags
    decr!("test"; "literal" => 1, "expression" => tag);
}
