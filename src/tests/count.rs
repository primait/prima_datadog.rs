use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::DatadogWrapper;
use crate::EMPTY_TAGS;

#[test]
pub fn count_with_literal() {
    let mock = mocks::count_mock("test", 10, EMPTY_TAGS);
    DatadogWrapper::new(mock, true).do_count("test", 10, EMPTY_TAGS);
}

#[test]
pub fn count_with_type() {
    let mock = mocks::count_mock("test1_event", 10, EMPTY_TAGS);
    DatadogWrapper::new(mock, true).do_count(TestEvent::Test1, 10, EMPTY_TAGS);
}

#[test]
pub fn count_with_literal_and_tags() {
    let mock = mocks::count_mock("test", 10, &["added:tag", "env:test"]);
    DatadogWrapper::new(mock, true).do_count("test", 10, &["added:tag"]);
}

#[test]
pub fn count_with_type_and_tags() {
    let mock = mocks::count_mock("test1_event", 10, &["added:tag", "env:test"]);
    DatadogWrapper::new(mock, true).do_count(TestEvent::Test1, 10, &["added:tag"]);
}
