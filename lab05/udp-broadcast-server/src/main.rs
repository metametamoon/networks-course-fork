use std::net::UdpSocket;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:1337").unwrap();
    udp_socket.set_read_timeout(Some(Duration::new(5, 0))).unwrap();
    udp_socket.set_broadcast(true).unwrap();
    // udp_socket.connect("255.255.255.255:0").unwrap();
    println!("Broadcast: {:?}", udp_socket.broadcast());
    loop {
        let ans = format!("Current time: {:?}", Instant::now());
        udp_socket.send_to(ans.as_bytes(), "255.255.255.255:0").unwrap();
        println!("[send] {}", ans);
        sleep(Duration::from_secs(2));
    }
}
