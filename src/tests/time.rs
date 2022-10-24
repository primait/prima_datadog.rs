use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;
use crate::TrackerConfiguration;

#[test]
pub fn time_with_literal() {
    let mock = mocks::time_mock("test", &[]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_time("test", vec![], || {});
}

#[test]
pub fn time_with_type() {
    let mock = mocks::time_mock("test1_event", &[]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_time(TestEvent::Test1, vec![], || {});
}

#[test]
pub fn time_with_literal_and_tags() {
    let mock = mocks::time_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_time("test", vec!["added:tag".to_string()], || {});
}

#[test]
pub fn time_with_type_and_tags() {
    let mock = mocks::time_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true, TrackerConfiguration::new()).do_time(
        TestEvent::Test1,
        vec!["added:tag".to_string()],
        || {},
    );
}
