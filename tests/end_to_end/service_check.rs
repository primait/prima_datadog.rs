use prima_datadog::{ServiceCheckOptions, ServiceStatus};
use serial_test::serial;

use crate::end_to_end::{init_test_datadog, read_string_from};

#[test]
#[serial]
fn test_service_check_with_option_and_key_value() {
    let socket = init_test_datadog();
    let options = ServiceCheckOptions {
        timestamp: Some(123),
        hostname: Some("localhost"),
        message: Some("message"),
    };

    prima_datadog::service_check!("test", ServiceStatus::Critical, options; "key" => "value");

    let service_check = read_string_from(socket);
    let expected = format!(
        "test|{}|d:123|h:localhost|m:message|#key:value",
        ServiceStatus::Critical as u32
    );
    assert!(service_check.contains(&expected));
}

#[test]
#[serial]
fn test_service_check_with_option() {
    let socket = init_test_datadog();
    let options = ServiceCheckOptions {
        timestamp: Some(123),
        hostname: Some("localhost"),
        message: Some("message"),
    };

    prima_datadog::service_check!("test", ServiceStatus::Critical, options);

    let service_check = read_string_from(socket);
    let expected = format!(
        "test|{}|d:123|h:localhost|m:message",
        ServiceStatus::Critical as u32
    );
    assert!(service_check.contains(&expected));
}

#[test]
#[serial]
fn test_service_check_with_path_as_input() {
    let socket = init_test_datadog();

    prima_datadog::service_check!("test", ServiceStatus::Critical);

    let service_check = read_string_from(socket);
    let expected = format!("test|{}", ServiceStatus::Critical as u32);
    assert!(service_check.contains(&expected));
}
