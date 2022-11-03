use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::EMPTY_TAGS;
use crate::{DatadogWrapper, ServiceCheckOptions, ServiceStatus};

#[test]
pub fn service_check_with_literal() {
    let mock = mocks::service_check_mock(
        "test",
        ServiceStatus::OK,
        EMPTY_TAGS,
        Some(ServiceCheckOptions::default()),
    );
    DatadogWrapper::new(mock, true).do_service_check(
        "test",
        ServiceStatus::OK,
        EMPTY_TAGS,
        Some(ServiceCheckOptions::default()),
    );
}

#[test]
pub fn service_check_with_type() {
    let mock = mocks::service_check_mock(
        "test1_event",
        ServiceStatus::OK,
        EMPTY_TAGS,
        Some(ServiceCheckOptions::default()),
    );
    DatadogWrapper::new(mock, true).do_service_check(
        TestEvent::Test1,
        ServiceStatus::OK,
        EMPTY_TAGS,
        Some(ServiceCheckOptions::default()),
    );
}

#[test]
pub fn service_check_with_literal_and_tags() {
    let mock = mocks::service_check_mock(
        "test",
        ServiceStatus::OK,
        &["added:tag", "env:test"],
        Some(ServiceCheckOptions::default()),
    );
    DatadogWrapper::new(mock, true).do_service_check(
        "test",
        ServiceStatus::OK,
        &["added:tag"],
        Some(ServiceCheckOptions::default()),
    );
}

#[test]
pub fn service_check_with_type_and_tags() {
    let mock = mocks::service_check_mock("test1_event", ServiceStatus::OK, &["added:tag", "env:test"], None);
    DatadogWrapper::new(mock, true).do_service_check(TestEvent::Test1, ServiceStatus::OK, &["added:tag"], None);
}
