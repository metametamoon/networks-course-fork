use std::fs::File;
use std::io::{ErrorKind, Write};
use std::net::UdpSocket;
use rand::Rng;
use utils::unwrap_packet;

const FILE_SIZE: i32 = 51255;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let port = args.get(1).map(|x| x.as_str()).unwrap_or("1337");
    let address = format!("127.0.0.1:{}", port);
    let server_address = format!("127.0.0.1:{}", 1338);
    let udp_socket =
        UdpSocket::bind(address).unwrap_or_else(|_| panic!("Failed to bind to port {}", port));
    println!("Running receiver at localhost:{}", port);
    let mut cur_expecting_bit = 0u8;
    let mut total_file_content = Vec::<u8>::new();
    
    loop {
        if total_file_content.len() == FILE_SIZE as usize {
            let mut output = File::create("result.png").unwrap();
            output.write_all(&total_file_content).unwrap();
            break;
        }
        let mut buf = [0u8; 1024];
        let res = udp_socket.recv(&mut buf);
        match res {
            Ok(len) => {
                let received_pkt = unwrap_packet(&buf[..len]);
                match received_pkt {
                    None => {udp_socket.send_to(&[0u8], &server_address).unwrap();}
                    Some(mut received_pkt) => {
                        if received_pkt.bit == cur_expecting_bit {
                            println!("Received payload len {}", len - 1);
                            total_file_content.append(&mut received_pkt.data);
                            println!("Cur file size: {}", total_file_content.len());
                            // println!("received: {}", String::from_utf8_lossy(payload));
                            cur_expecting_bit = 1 - cur_expecting_bit;
                            udp_socket.send_to(&[1u8], &server_address).unwrap(); // ack
                        }
                    }
                }
                // if buf[0] == cur_expecting_bit {
                //     println!("Received payload len {}", len - 1);
                //     let payload = &buf[1..len];
                //     total_file_content.append(&mut payload.to_vec());
                //     println!("Cur file size: {}", total_file_content.len());
                //     // println!("received: {}", String::from_utf8_lossy(payload));
                //     cur_expecting_bit = 1 - cur_expecting_bit;
                //     udp_socket.send_to(&[1u8], &server_address).unwrap(); // ack
                // } else {
                //     udp_socket.send_to(&[0u8], &server_address).unwrap();
                // }
            }
            Err(err) => match err.kind() {
                ErrorKind::ConnectionReset => {
                    println!("End of communication");
                }
                _ => {
                    panic!("Unexpected error! {:?}", err)
                }
            },
        }
    }
}

fn send_unsafe(udp_socket: &UdpSocket, receiver_address: &String, raw_pkt: &[u8]) -> usize {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..10) < 5 {
        udp_socket.send_to(raw_pkt, receiver_address).unwrap()
    } else {
        println!("Ooops!");
        raw_pkt.len()
    }
}