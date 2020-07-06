// This program uploads selected files to an M5stack
use std::fs::ReadDir;
use std::fs::{create_dir, read_dir, DirEntry, File};
use std::path::Path;
use std::io::{Read, stdin};
fn main() {
    //Entry scanner
    let path = Path::new("deploy/");
    let dir_data_unsafe = read_dir(path);
    if dir_data_unsafe.is_ok() {
        println!("Deploy directory found!");
    } else {
        create_dir(path).expect("Could not autocrete 'deploy' directory");
    }
    let dir_data = dir_data_unsafe.unwrap();
    let files: Vec<DirEntry> = dir_data
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap())
        .collect();
    let mut counter = 0;
    for file in files { 
        let filename = file.file_name().into_string().unwrap();
        println!("{}) {}",counter, filename);
        counter += 1;
    }

    //user input
    let mut input = String::new();
    loop {
        
    }
    std::io::stdin().read_line(&mut input).expect("could not read input");
    
}
