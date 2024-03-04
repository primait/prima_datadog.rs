mod event;
mod service_check;

use once_cell::sync::OnceCell;
use prima_datadog::{configuration::Config, Datadog};
use std::net::UdpSocket;

static SOCKET: OnceCell<UdpSocket> = OnceCell::new();

fn read_string_from(socket: &UdpSocket) -> String {
    let mut buf = [0; 100];
    let (length, _) = socket.recv_from(&mut buf).expect("Could not read from socket");
    let buf = &buf[..length];
    String::from_utf8(buf.to_vec()).unwrap()
}

fn init_test_datadog() -> &'static UdpSocket {
    let socket = SOCKET.get_or_init(|| UdpSocket::bind("127.0.0.1:0").expect("couldn't open udp socket"));
    let address_to = format!("127.0.0.1:{}", socket.local_addr().unwrap().port());

    let configuration = Config::new(&address_to, "prova_datadog");
    let _ = Datadog::init(configuration);
    socket
}
