use std::io::ErrorKind;
use std::net::UdpSocket;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let port = args
        .get(1)
        .map(|x| x.as_str())
        .unwrap_or("1338");
    let address = format!("127.0.0.1:{}", port);
    let udp_socket = UdpSocket::bind(&address).expect(&format!("Failed to bind to port {}", port));
    println!("Running at localhost:{}", port);
    udp_socket.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
    let mut failed_attempts = 0;
    let attempts = 20;
    let mut rtts = Vec::<f32>::new();
    for i in 0..attempts {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let message = format!("ping {} {}", i, current_time);

        let timer = Instant::now();
        udp_socket.send_to(message.as_bytes(), &"127.0.0.1:1337").unwrap();
        let mut buf = [0u8; 256];
        match udp_socket.recv(&mut buf) {
            Ok(amt) => {
                let rtt = timer.elapsed().as_secs_f32();
                println!("{}; rtt={}s", String::from_utf8_lossy(&buf[..amt]), rtt);
                rtts.push(rtt);
            }
            Err(error) => {
                if error.kind() == ErrorKind::TimedOut {
                    println!("{}: timeout", i);
                } else {
                    panic!("Unexpected error: {}", error);
                }
                failed_attempts += 1
            }
        }
    }
    print_statistics(&attempts, &failed_attempts, &rtts);
}

fn print_statistics(attempts: &i32, failed_attempts: &i32, rtts: &Vec<f32>) {
    println!("Loss ratio = {}/{} = {}", failed_attempts, attempts, (*failed_attempts as f32) / (*attempts as f32));
    println!("min_rtt = {}s", rtts.iter().min_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
    println!("max_rtt = {}s", rtts.iter().max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap());
    let rtts_len = rtts.len();
    println!("avg_rtt = {}s", rtts.iter().sum::<f32>() / rtts_len as f32);
}
