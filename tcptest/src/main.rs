use std::net::{TcpStream, TcpListener};
use std::io::{Read,Write};
use std::str;
use std::char;
fn main() {
  
    let  listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut buffer: [u8;1024] = [0;1024];
    for stream in listener.incoming() {
        let mut data = stream.unwrap();
        data.read(&mut buffer).unwrap();
        let message: Vec<u8> = buffer.iter()
                                .filter(|c| **c as u32 != 0)
                                .map(|c| *c)
                                .collect();
        println!("{:?}", str::from_utf8(&message));
        buffer = [0;1024];
    }
}
