use crate::tests::mocks;
use crate::tests::TestEvent;
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
