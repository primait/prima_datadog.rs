use crate::service_check;
use crate::tests::mocks;
use crate::tests::TestEvent;
use crate::TagTrackerConfiguration;
use crate::EMPTY_TAGS;
use crate::{Datadog, ServiceCheckOptions, ServiceStatus};

#[test]
pub fn service_check_with_literal() {
    let mock = mocks::service_check_mock("test", ServiceStatus::OK, &[], Some(ServiceCheckOptions::default()));
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_service_check(
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
        &[],
        Some(ServiceCheckOptions::default()),
    );
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_service_check(
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
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_service_check(
        "test",
        ServiceStatus::OK,
        vec!["added:tag".to_string()],
        Some(ServiceCheckOptions::default()),
    );
}

#[test]
pub fn service_check_with_type_and_tags() {
    let mock = mocks::service_check_mock("test1_event", ServiceStatus::OK, &["added:tag", "env:test"], None);
    Datadog::new(mock, true, TagTrackerConfiguration::new()).do_service_check(
        TestEvent::Test1,
        ServiceStatus::OK,
        vec!["added:tag".to_string()],
        None,
    );
}

#[test]
fn test_macro() {
    let tag = String::from("tag");
    // no tags
    service_check!("test", ServiceStatus::OK);
    // just literal tags
    service_check!("test", ServiceStatus::OK; "literal" => 1);
    // just expression tags
    service_check!("test", ServiceStatus::OK; "expression" => tag);
    // mixed tags
    service_check!("test", ServiceStatus::OK; "literal" => 1, "expression" => tag);
    // no tags with options
    service_check!("test", ServiceStatus::OK, ServiceCheckOptions::default());
    // just literal tags with options
    service_check!("test", ServiceStatus::OK, ServiceCheckOptions::default(); "literal" => 1);
    // just expression tags with options
    service_check!("test", ServiceStatus::OK, ServiceCheckOptions::default(); "expression" => tag);
    // mixed tags with options
    service_check!(
        "test",
        ServiceStatus::OK,
        ServiceCheckOptions::default();
        "literal" => 1,
        "expression" => tag
    );
}
