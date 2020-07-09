// This program uploads selected files to an M5stack
use std::net::{TcpListener,TcpStream};
use std::fs::{create_dir, read_dir, read_to_string, DirEntry};
use std::path::Path;
use std::io::{Read, Write};
use std::{str,char};
const LOCAL: &str = "127.0.0.1:7878";
fn main() {
    
    //Dir entry scanner
    let path = Path::new("deploy/");
    let dir_data_unsafe = read_dir(path);
    if dir_data_unsafe.is_ok() {
        println!("Deploy directory found!");
    } else {
        create_dir(path).expect("Could not autocreate deploy directory");
    }
    let dir_data = dir_data_unsafe.unwrap();
    let files: Vec<DirEntry> = dir_data
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .collect();

    let mut counter = 0;

    for file in &files {
        let filename = file.file_name().into_string().unwrap();
        println!("{}) {}", counter, filename);
        counter += 1;
    }

    //user input
    let mut input = String::new();
    
    let selection_index: usize;
    
    loop {
        println!("Select a file (number)");
        std::io::stdin()
            .read_line(&mut input)
            .expect("could not read input");
        let parsed_unsafe = input.trim().parse::<usize>();
        if parsed_unsafe.is_ok() {
    
            if parsed_unsafe.clone().unwrap() <= counter - 1 {
                //Correct input
                selection_index = parsed_unsafe.unwrap();
                break;
            } else {
                println!("Incorrect input, must be less or equal to number of files");
            }
        } else {
            println!("Invalid input");
    
        }
        input.clear();
    }
    //File selection fetching
    let file_string = read_to_string(files[selection_index]
        .path())
        .expect("Could not read file");
    println!("{:?}",file_string.into_bytes());

    let mut listener = TcpListener::bind(LOCAL).expect("Could not bind");
    let mut buffer: [u8;1024] = [0;1024]; //1kb
    for stream in listener.incoming() {
        let mut stream2 = stream.unwrap();
        stream2.read(&mut buffer).unwrap();
        let message: String = buffer.into_iter()
                        .map(|c| char::from_digit(*c as u32, 10).unwrap())
                        .collect();
        println!("{}", message);
        buffer = [0;1024];
    }
}

