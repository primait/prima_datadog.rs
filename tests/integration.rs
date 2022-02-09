use once_cell::sync::OnceCell;
use prima_datadog::{configuration::PrimaConfiguration, Datadog, ServiceStatus};
use serial_test::serial;
use std::net::UdpSocket;

static SOCKET: OnceCell<UdpSocket> = OnceCell::new();

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

#[test]
#[serial]
fn test_service_check_with_path_as_input() {
    let socket = init_test_datadog();

    prima_datadog::service_check!("test", ServiceStatus::Critical);

    let check = read_as_string(socket);

    let expected = format!("test|{}", ServiceStatus::Critical as u32);
    assert!(check.contains(&expected));
}

fn read_as_string(socket: &UdpSocket) -> String {
    let mut buf = [0; 50];
    let (length, _) = socket
        .recv_from(&mut buf)
        .expect("Could not read from socket");
    let buf = &buf[..length];
    String::from_utf8(buf.to_vec()).unwrap()
}

fn init_test_datadog() -> &'static UdpSocket {
    let socket =
        SOCKET.get_or_init(|| UdpSocket::bind("127.0.0.1:0").expect("couldn't open udp socket"));
    let address_to = format!("127.0.0.1:{}", socket.local_addr().unwrap().port());

    let configuration = PrimaConfiguration::new(
        &address_to,
        "0.0.0.0:0",
        "prova_datadog",
        "production".parse().unwrap(),
    );
    let _ = Datadog::init(configuration);
    socket
}
