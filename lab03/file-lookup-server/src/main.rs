mod threadpool;

use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::*,
    path::Path,
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

use crate::threadpool::ThreadPool;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse().unwrap()
    } else {
        1337
    };
    let thread_pool = ThreadPool::new(6);
    println!("Opened at port {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread_pool.execute(|| handle_connection(stream));
        // thread::spawn(|| handle_connection(stream));
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let reader = BufReader::new(&mut stream);
        let request = reader
            .lines()
            .map(|x| x.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();   
        if request.len() == 0 {
            continue;
        }
        let main_part = &request[0];
        println!("main_part={main_part}");
        let file_name = main_part
            .trim()
            .strip_prefix("GET /")
            .and_then(|x| x.strip_suffix("HTTP/1.1"));
        println!("{file_name:?}");
        if let Some(file_name) = file_name {
            if !file_name.starts_with("favicon") {
                let path = Path::new(file_name);
                if let Ok(mut file) = File::open(path) {
                    let status = "HTTP/1.1 200 OK";
                    let mut contents: String = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    let length = contents.len();
                    let response =
                        format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
                    println!("{}", response);
                    stream.write_all(response.as_bytes()).unwrap();
                    continue;;
                }
            } else {
                let status = "HTTP/1.1 200 OK";
                println!("{}", status);
                stream.write_all(status.as_bytes()).unwrap();
                continue;
            }
        }
        let status = "HTTP/1.1 404 Not Found";
        println!("{}", status);
        stream.write_all(status.as_bytes()).unwrap();
    }
}
