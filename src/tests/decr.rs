use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::Datadog;

#[test]
pub fn decr_with_literal() {
    let mock = mocks::decr_mock("test", &[]);
    Datadog::new(mock, true).do_decr("test", &[]);
}

#[test]
pub fn decr_with_type() {
    let mock = mocks::decr_mock("test1_event", &[]);
    Datadog::new(mock, true).do_decr(TestEvent::Test1, &[]);
}

#[test]
pub fn decr_with_literal_and_tags() {
    let mock = mocks::decr_mock("test", &["added:tag", "env:test"]);
    Datadog::new(mock, true).do_decr("test", &["added:tag"]);
}

#[test]
pub fn decr_with_type_and_tags() {
    let mock = mocks::decr_mock("test1_event", &["added:tag", "env:test"]);
    Datadog::new(mock, true).do_decr(TestEvent::Test1, &["added:tag"]);
}
