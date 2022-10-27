use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::DatadogWrapper;

#[test]
pub fn incr_with_literal() {
    let mock = mocks::incr_mock("test", &[]);
    DatadogWrapper::new(mock, true).do_incr("test", &[]);
}

#[test]
pub fn incr_with_type() {
    let mock = mocks::incr_mock("test1_event", &[]);
    DatadogWrapper::new(mock, true).do_incr(TestEvent::Test1, &[]);
}

#[test]
pub fn incr_with_literal_and_tags() {
    let mock = mocks::incr_mock("test", &["added:tag", "env:test"]);
    DatadogWrapper::new(mock, true).do_incr("test", &["added:tag"]);
}

#[test]
pub fn incr_with_type_and_tags() {
    let mock = mocks::incr_mock("test1_event", &["added:tag", "env:test"]);
    DatadogWrapper::new(mock, true).do_incr(TestEvent::Test1, &["added:tag"]);
}
