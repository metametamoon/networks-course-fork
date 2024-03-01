use std::{io::{Read, Write}, net::TcpStream, str::from_utf8};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let defaul_host = "127.0.0.1".to_owned();
    let defaul_port = "1337".to_owned();
    let defaul_file_name = "b.txt".to_owned();
    
    let host = args.get(1).unwrap_or(&defaul_host);
    let port = args.get(2).unwrap_or(&defaul_port);
    let file_name = args.get(3).unwrap_or(&defaul_file_name);
    
    let addr = format!("{}:{}", host, port);
    println!("Connecting to {}", addr);
    let mut stream = TcpStream::connect(addr).unwrap();

    let query = format!("GET /{} HTTP/1.1\r\n\r\n", file_name);
    stream.write_all(query.as_bytes()).unwrap();
    let mut response =  Vec::<u8>::new();
    loop {
        let mut buf = [0u8; 128];
        let read = stream.read(&mut buf).unwrap();
        if read == 0 {
            break;
        } else {
            response.extend(&buf[0..read]);
            println!("Received another {read} bytes");
            if read < 128 {
                break;
            }
        }
    }
    let response_str = from_utf8(&response);
    if let Ok(str) = response_str {
        println!("{}", str);
    } else {
        println!("Not utf8; too bad!");
    }
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
