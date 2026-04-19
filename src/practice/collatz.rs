use std::convert::TryInto;
use crate::practice::helper::{flush_stdout, read_number};

fn collatz_length(mut n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    let mut answer: u32 = 1;
    while n > 1 {
        n = if n&1 == 1 { 3*n + 1 } else { n >> 1 };
        answer+=1;
    }
    answer
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

pub fn run(){
    flush_stdout("Enter a number for Collatz >> ");
    let n: Result<u32, _> = read_number().try_into();
    match n {
        Ok(n) => {
            println!("length of collatz: {}", collatz_length(n))
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }

}