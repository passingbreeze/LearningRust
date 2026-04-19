use std::time::{Duration, Instant};
use std::io;

const MAX_LEN : u32 = 100_000_000;

pub fn run() {
    let start: Instant = Instant::now();

    let mut sieve: Vec<bool> = vec![false; MAX_LEN as usize];
    let mut input_str: String = String::new();

    println!("Enter a number to find primes up to:");
    io::stdin().read_line(&mut input_str).expect("input Failure");

    let target_num: u32 = input_str.trim().parse().expect("Only Unsigned Integer");

    if target_num < MAX_LEN {
        find_primes(&mut sieve, target_num);
    } else {
        println!("Input Number is greater than MAXSIZE");
    }

    let duration: Duration = start.elapsed();
    println!("\nexec time : {:?}", duration);
}

fn find_primes(sieve: &mut [bool], to: u32) {
    let to = to as usize;

    let mut p = 2;
    while p * p <= to {
        if !sieve[p] {
            let mut i = p * p;
            while i <= to {
                sieve[i] = true;
                i += p;
            }
        }
        p += 1;
    }

    for i in 2..=to {
        if !sieve[i] {
            print!("{} ", i);
        }
    }
    println!();
}