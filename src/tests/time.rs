use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;

#[test]
pub fn time_with_literal() {
    let mock = mocks::time_mock("test", &[]);
    Datadog::new(mock, true).do_time("test", &[], || {});
}

#[test]
pub fn time_with_type() {
    let mock = mocks::time_mock("test1_event", &[]);
    Datadog::new(mock, true).do_time(TestEvent::Test1, &[], || {});
}

#[test]
pub fn time_with_literal_and_tags() {
    let mock = mocks::time_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true).do_time("test", &["added:tag"], || {});
}

#[test]
pub fn time_with_type_and_tags() {
    let mock = mocks::time_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true).do_time(TestEvent::Test1, &["added:tag"], || {});
}
