use std::net::UdpSocket;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let port = args
        .get(1)
        .map(|x| x.as_str())
        .unwrap_or("1337");
    let address = format!("127.0.0.1:{}", port);
    let udp_socket = UdpSocket::bind(&address).expect(&format!("Failed to bind to port {}", port));
    println!("Running at localhost:{}", port);
    let mut iter = 0;
    loop {
        let mut buf = [0u8; 256];
        let (amt, address) = udp_socket.recv_from(&mut buf).unwrap();
        println!("Received msg=[{}]", String::from_utf8_lossy(&buf[..amt]));
        if iter % 5 != 0 {
            udp_socket.send_to(&buf[..amt], &address).unwrap();
        }
        iter += 1;
    }
}
