use std::io;
use std::time::Instant;

fn main(){
    let start = Instant::now();

    let mut s = "hello";

    println!("{}",s);


    println!("\nexec time : {:?}", start.elapsed());
}