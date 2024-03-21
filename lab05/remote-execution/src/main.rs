use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, process::Command};

fn handle_connection(stream: &mut TcpStream) {
    let mut buf = [0u8; 128];
    let read_symbols = stream.read(&mut buf);
    match read_symbols {
        Ok(read) => {
            let input_command_line = String::from_utf8_lossy(&buf[..read]);
            let tokens = input_command_line.split(' ').collect::<Vec<_>>();
            if tokens.len() == 0 || tokens[0].len() == 0 {
                stream.write_all(b"Bad command").unwrap();
            } else {
                let command = tokens[0];
                let args = &tokens[1..];
                let output = Command::new(command)
                    .args(args)
                    .output()
                    .expect("failed to execute process");
                let stddout = output.stdout;
                stream.write_all(&stddout).unwrap();
            }

        },
        Err(_) => {
            println!("Error on read (interrupted)")
        },
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let default_port = "1337".to_string();
    let port = args.get(2).unwrap_or(&default_port);
    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).unwrap();
    for incoming in listener.incoming() {
        let mut connection = incoming.unwrap();
        handle_connection(&mut connection);
    }

}
