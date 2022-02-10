use serial_test::serial;

use crate::end_to_end::{init_test_datadog, read_as_string};

#[test]
#[serial]
fn test_event_notification_with_variable_as_description() {
    let socket = init_test_datadog();

    let variable = "variabile";
    prima_datadog::event!("test", variable);

    let event = read_as_string(socket);

    assert!(event.contains(variable))
}

#[test]
#[serial]
fn test_event_notification_with_literal_as_description() {
    let socket = init_test_datadog();

    prima_datadog::event!("test", "variable");

    let event = read_as_string(socket);
    assert!(event.contains("variable"))
}
