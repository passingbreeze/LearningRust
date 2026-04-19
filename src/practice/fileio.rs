use std::time::Instant;
use std::fs::File;
use std::io::{ErrorKind, Write};

pub fn run() {
    let start = Instant::now();
    
    let mut f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").expect("Problem creating the file")
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    f.write_all(b"Hello, world!").expect("Failed to write to file");

    println!("\nexec time : {:?}", start.elapsed());
}