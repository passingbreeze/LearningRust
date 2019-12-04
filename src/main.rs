use std::time::{Duration, Instant};
use std::io;

const MAX_LEN : u32 = 100_000_000;

fn main(){
    let start : Instant = Instant::now();
    let a = [false; MAX_LEN];
    let duration : Duration = start.elapsed();
    println!("\nexec time : {:?}", duration);
}