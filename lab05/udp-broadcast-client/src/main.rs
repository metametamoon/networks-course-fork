use std::net::UdpSocket;

fn main() {
    let connection = UdpSocket::bind("127.0.0.1:1338").unwrap();
    connection.set_broadcast(true).unwrap();
    println!("Broadcast: {:?}", connection.broadcast());
    loop {
        let mut buf = [0u8; 512];
        let result = connection.recv(&mut buf);
        match result {
            Ok(read) => { println!("{}", String::from_utf8_lossy(&buf[..read])) }
            Err(_) => {
                println!("Nothing received")
            }
        }
    }
}
