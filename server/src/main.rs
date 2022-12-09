use std::io::{BufRead, Read};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Runing on port 8080");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        // read message in stream, and storage in buffer
        stream.read(&mut buffer).unwrap();

        let mut message= String::from_utf8(buffer.to_vec()).unwrap();
        // only need the string
        let message: Vec<&str> = message.split("\0").collect();
        println!("message is {}", message[0]);
    }
}