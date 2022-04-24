mod practice;

use std::time::{Instant,Duration};
use std::error::Error;
use practice::thread_test;

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    thread_test::thread_test();

    println!("\nexec time : {:?}", start.elapsed());
    Ok(())
}