use std::fs::{File, read_to_string};
use std::io::{Read, Write};
use std::net::*;
use std::str::{from_utf8};

fn is_blacklisted(addr: &str) -> bool {
    let evil_corp = "google";
    if addr.contains(evil_corp) {
        return true;
    }
    let content = read_to_string("blacklist.txt").unwrap();
    let mut lines = content.lines();
    lines.any(|x| x == addr)
}

fn redirect_request(addr: &str, get_request: &str) -> Option<String> {
    println!("addr={addr} get_req={get_request}");
    if is_blacklisted(addr) {
        return Some("HTTP/1.1 403 Forbidden\r\n\r\n".to_string());
    }
    let mut stream = TcpStream::connect(format!("{addr}:80").as_str()).ok()?;
    let request = format!("GET /{get_request} HTTP/1.1\r\nHost: {addr}\r\n\r\n");
    println!("request={request}");
    stream.write_all(request.as_bytes()).unwrap();
    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf);
    match result {
        Ok(_) => {
            File::create("out.html").unwrap().write_all(buf.as_bytes()).unwrap();
            Some(buf)
        }
        Err(err) => {
            println!("Error {:?}", err);
            None
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let default_port = "1337".to_string();
    let port = args.get(1).unwrap_or(&default_port);
    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on {addr}");
    for incoming in listener.incoming() {
        let mut connection = incoming.unwrap();
        handle_connection(&mut connection);
    }
}

fn handle_connection(connection: &mut TcpStream) {
    println!("Accepted connection form {:?}", connection.peer_addr());
    let result = {
        let mut buf = [0u8; 4096];
        if let Ok(read) = connection.read(&mut buf) {
            println!("Read {read} bytes");
            from_utf8(&buf[..read]).unwrap().to_owned()
        } else {
            String::new()
        }
    };
    println!("Incoming request: {}", result);
    let lines = result.split("\r\n").collect::<Vec<_>>();
    let get_query_line = lines[0];
    if get_query_line.contains("favicon") {
        connection.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
        return;
    }
    let get_query = get_query_line.strip_prefix("GET /").and_then(|x| x.strip_suffix(" HTTP/1.1"));
    if let Some(get_query) = get_query {
        let (addr, get_req) = get_query.split_once('/')
            .unwrap_or((get_query, ""));
        let maybe_response = redirect_request(addr, get_req);
        match maybe_response {
            Some(response) => {
                println!("Got a response");
                connection.write_all(response.as_bytes()).unwrap();
                println!("Sent a response");
            }
            None => {
                println!("Failed to reach");
            }
        }
    }
    connection.shutdown(Shutdown::Both).unwrap()
}
