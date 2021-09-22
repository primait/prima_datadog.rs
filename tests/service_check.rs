mod common;
mod mocks;

use prima_datadog::{Datadog, ServiceCheckOptions, ServiceStatus};

#[test]
pub fn service_check_with_literal() {
    let mock = mocks::service_check_mock(
        "test",
        ServiceStatus::OK,
        &[],
        Some(ServiceCheckOptions::default()),
    );
    Datadog::new(mock, true, vec![]).service_check(
        "test",
        ServiceStatus::OK,
        vec![],
        Some(ServiceCheckOptions::default()),
    );
}

#[test]
pub fn service_check_with_type() {
    let mock = mocks::service_check_mock(
        "test1_event",
        ServiceStatus::OK,
        &[],
        Some(ServiceCheckOptions::default()),
    );
    Datadog::new(mock, true, vec![]).service_check(
        common::TestEvent::Test1,
        ServiceStatus::OK,
        vec![],
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
    Datadog::new(mock, true, ["env:test"]).service_check(
        "test",
        ServiceStatus::OK,
        vec!["added:tag".to_string()],
        Some(ServiceCheckOptions::default()),
    );
}

#[test]
pub fn service_check_with_type_and_tags() {
    let mock = mocks::service_check_mock(
        "test1_event",
        ServiceStatus::OK,
        &["added:tag", "env:test"],
        None,
    );
    Datadog::new(mock, true, ["env:test"]).service_check(
        common::TestEvent::Test1,
        ServiceStatus::OK,
        vec!["added:tag".to_string()],
        None,
    );
}
