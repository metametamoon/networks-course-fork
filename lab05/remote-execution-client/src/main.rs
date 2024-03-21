use std::{io::{Read, Write}, net::TcpStream};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let default_port = "1337".to_string();
    let port = args.get(2).unwrap_or(&default_port);
    let address = format!("127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(&address).unwrap();
    stream.write_all(b"ping yandex.ru").unwrap();
    let mut buf = [0u8; 1024];
    if let Ok(_) = stream.read(&mut buf) {
        println!("{}", String::from_utf8_lossy(&buf));
    }    

}
