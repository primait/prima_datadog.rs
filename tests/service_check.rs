mod common;
mod mocks;

use prima_datadog::{Datadog, ServiceCheckOptions, ServiceStatus};

#[test]
pub fn service_check_with_literal() {
    let mock = mocks::service_check_mock("test", ServiceStatus::OK, &[], Some(ServiceCheckOptions::default()));
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
    let mock = mocks::service_check_mock("test1_event", ServiceStatus::OK, &["added:tag", "env:test"], None);
    Datadog::new(mock, true, ["env:test"]).service_check(
        common::TestEvent::Test1,
        ServiceStatus::OK,
        vec!["added:tag".to_string()],
        None,
    );
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_service_check_with_literal() {
    prima_datadog::service_check!("test", ServiceStatus::OK, Some(ServiceCheckOptions::default()));
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_service_check_with_type() {
    use common::TestEvent;
    prima_datadog::service_check!(
        TestEvent::Test1,
        ServiceStatus::OK,
        Some(ServiceCheckOptions::default())
    );
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_service_check_with_literal_and_tags() {
    prima_datadog::service_check!(
        "test",
        ServiceStatus::OK,
        Some(ServiceCheckOptions::default());
        "added" => "tag"
    );
}

#[test]
#[cfg(feature = "noop")]
pub fn macro_service_check_with_type_and_tags() {
    use common::{TestEvent, TestEvent2};
    prima_datadog::service_check!(
        TestEvent::Test1,
        ServiceStatus::OK,
        None as Option<ServiceCheckOptions>; // TODO: maybe there's a better way to solve type annotations needed for `Option<T>`?
        TestEvent2::Test2 => "tag"
    );
}
