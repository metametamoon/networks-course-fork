use std::collections::VecDeque;
use std::fmt::format;
use std::fs::File;
use std::io::Read;

use rand::Rng;
use std::net::UdpSocket;
use std::time::{Duration, Instant};
use utils::create_packet;

enum ReceiverState {
    AwaitingSend {
        bit: u8,
    },
    AwaitingAck {
        bit: u8,
        start_time: Instant,
        duration: Duration,
        last_sent_pkt: Vec<u8>,
    },
}

const ACK: u8 = 1u8;
const FILE_SIZE: i32 = 51255;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let port = args.get(1).map(|x| x.as_str()).unwrap_or("1338");
    let receiver_address = format!("127.0.0.1:{}", "1337");
    let address = format!("127.0.0.1:{}", port);
    let udp_socket = UdpSocket::bind(&address).expect(&format!("Failed to bind to port {}", port));
    println!("Running sender at localhost:{}", port);

    let mut file = File::open("pegasus.png").unwrap();
    let mut file_content = Vec::<u8>::new();
    file.read_to_end(&mut file_content).unwrap();
    dbg!(file_content.len());
    let packet_len = 1000;
    let mut packet_queue = file_content.chunks(packet_len as usize).map
    (|x| x.to_vec()).collect::<VecDeque<_>>();
    let packed: usize = packet_queue.iter().map(|x| x.len()).sum();
    dbg!(packed);
    
    let mut received_queue = VecDeque::<Vec<u8>>::new();
    let mut state = ReceiverState::AwaitingSend { bit: 0 };
    loop {
        let ack_received = step_rdt(
            &udp_socket,
            &mut packet_queue,
            &mut received_queue,
            &mut state,
            &receiver_address,
        );
        if packet_queue.is_empty() && ack_received {
            break;
        }
        // println!("Another iter of algorithm!")
    }
}

// returns true only if ack was received; otherwise, returns false
fn step_rdt(
    udp_socket: &UdpSocket,
    packet_queue: &mut VecDeque<Vec<u8>>,
    received_queue: &mut VecDeque<Vec<u8>>,
    state: &mut ReceiverState,
    receiver_address: &String,
) -> bool {
    match state {
        ReceiverState::AwaitingSend { bit } => {
            let bit = *bit;
            let Some(front) = packet_queue.pop_front() else {
                return false;
            };
            let pkt = create_packet(bit, &front);
            let res = send_unsafe(udp_socket, receiver_address, &pkt);
            assert_eq!(pkt.len(), res);
            *state = ReceiverState::AwaitingAck {
                bit,
                start_time: Instant::now(),
                duration: Duration::from_millis(10),
                last_sent_pkt: pkt,
            };
            println!("Send at bit {}; now awaiting ack.", bit);
        }
        ReceiverState::AwaitingAck {
            bit,
            start_time,
            duration,
            ref last_sent_pkt,
        } => {
            let bit = *bit;
            let duration = *duration;
            let start_time = *start_time;
            let elapsed_since_entering_state = start_time.elapsed();
            let diff = duration.saturating_sub(elapsed_since_entering_state);

            if diff.is_zero() {
                // resend
                println!("Timeout; resending");
                udp_socket.set_read_timeout(Some(duration)).expect("what?");
                let res = send_unsafe(udp_socket, receiver_address, &last_sent_pkt);
                assert_eq!(last_sent_pkt.len(), res);
                *state = ReceiverState::AwaitingAck {
                    bit,
                    start_time: Instant::now(),
                    duration,
                    last_sent_pkt: last_sent_pkt.clone(),
                }
            } else {
                udp_socket.set_read_timeout(Some(diff)).expect("what?");

                let mut buf = [0u8; 256];
                let res = udp_socket.recv(&mut buf);
                match res {
                    Ok(_len) => {
                        let ack_field = buf[0];
                        if ack_field == ACK {
                            received_queue.push_back(buf[2..].to_owned());
                            *state = ReceiverState::AwaitingSend { bit: 1 - bit };
                            println!("ack received");
                            return true;
                        }
                    }
                    Err(_) => {
                        // do nothing, handle timeout at the beginning
                    }
                }
            }
        }
    }
    false
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
