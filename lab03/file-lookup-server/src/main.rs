use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::*,
    path::Path,
};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        1337
    };
    println!("Opened at port {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&mut stream);
    let request = reader
        .lines()
        .map(|x| x.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let main_part = &request[0];
    let file_name = main_part
        .strip_prefix("GET /")
        .and_then(|x| x.strip_suffix("HTTP/1.1"));
    if let Some(file_name) = file_name {
        if !file_name.starts_with("favicon") {
            let path = Path::new(file_name);
            if let Ok(mut file) = File::open(path) {
                let status = "HTTP/1.1 200 OK";
                let mut contents: String = String::new();
                file.read_to_string(&mut contents).unwrap();
                let length = contents.len();
                let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
                stream.write_all(response.as_bytes()).unwrap();
                return;
            }
        } else {
            let status = "HTTP/1.1 200 OK";
            stream.write_all(status.as_bytes()).unwrap();
            return;
        }
    }
    let status = "HTTP/1.1 404 Not Found";
    stream.write_all(status.as_bytes()).unwrap();
}
