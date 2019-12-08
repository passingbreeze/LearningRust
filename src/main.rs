use std::time::{Duration, Instant};
use std::{io, vec};
use std::io::Read;
use std::f32::MANTISSA_DIGITS;

const MAX_LEN : usize = 100_000_000;

fn main(){
    let start : Instant = Instant::now();

    let mut a : Vec<bool> = vec![false; MAX_LEN];
    let mut inpnum : String = String::new();

    io::stdin().read_line(&mut inpnum).expect("input Failure");

    let inpnum : u32 = inpnum.trim().parse().expect("Only Unsigned Integer");

    if inpnum < MAX_LEN as u32 {
        findPrime(&mut a, inpnum);
    }
    else {
        println!("Input Number is greater than MAXSIZE");
    }

    let duration : Duration = start.elapsed();
    println!("\nexec time : {:?}", duration);
}
//
fn findPrime(arr : &mut Vec<bool>, to : u32) {
    let mut from : u32 = 2;

    while (from*from) <= to {
        let mut j = from;
        while j<=to {
            if !arr[j] {
                arr[j] = true;
            }

        }
        from+=1;
    }

}