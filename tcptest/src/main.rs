use std::net::{TcpStream, TcpListener};
use std::io::Write;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    // let listener = TcpListener::bind("127.0.0.1:2020").expect("error");
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("error connecting to stream");

    let msg = &args[0];

    stream.write(&msg.as_bytes()).unwrap();
    

}
