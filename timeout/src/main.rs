use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::time::Duration;
use std::thread;

const SLEEP_SECONDS: u64 = 60;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n");

    println!("sleeping for {} seconds", SLEEP_SECONDS);
    let duration = Duration::new(SLEEP_SECONDS, 0);
    thread::sleep(duration);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("done");
}
