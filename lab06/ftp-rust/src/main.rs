use std::fmt::format;
use std::fs::File;
use std::io::Write;
use ftp::FtpStream;

fn handle_ftp_stream(ftp_stream: &mut FtpStream) {
    // print cws
    println!("cwd:");
    if let Ok(list) = ftp_stream.list(None) {
        for item in list {
            println!("{}", item)
        }
    }
    // upload file on server
    println!("Uploading file on server:");
    let upload_filename = "client-stuff.txt";
    ftp_stream.put(upload_filename, &mut File::open(upload_filename).unwrap())
        .expect("TODO: panic message");
    let download_filename = "server-stuff.txt";
    let downloaded = ftp_stream.simple_retr(download_filename).unwrap();
    File::create(download_filename)
        .unwrap()
        .write(downloaded.get_ref().as_slice())
        .expect("Failed to write");
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let ip = args.get(1).map(|x| x.as_str()).unwrap_or("127.0.0.1");
    let port = args.get(2).map(|x| x.as_str()).unwrap_or("21");
    let address = format!("{}:{}", ip, port);
    let mut ftp_stream = FtpStream::connect(&address).unwrap_or_else(|err|
        panic!("{}", err)
    );
    match ftp_stream.login("FtpUser", "123456") {
        Ok(_) => {
            handle_ftp_stream(&mut ftp_stream)
        }
        Err(err) => { println!("Err: {:?}", err) }
    }
    let _ = ftp_stream.quit();
}
