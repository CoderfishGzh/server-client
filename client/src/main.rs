use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write("hello rust".as_bytes()).unwrap();
}