use std::io;
use std::time::Instant;
use std::fs::File;
use std::io::{ErrorKind, Write};

fn main(){
    let start = Instant::now();
    let hellofile = File::open("hello.txt");

    let mut f= match hellofile {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    f.write_all(b"Hello, world!");

    println!("\nexec time : {:?}", start.elapsed());
}