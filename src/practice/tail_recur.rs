use std::io;
use std::io::Write;

pub fn run() {
    let mut num = String::new();

    print!("Enter number >> ");
    io::stdout().flush().expect("Flush Failed!");

    io::stdin().read_line(&mut num).expect("Input Failure.");
    let num: i32 = num.trim().parse().expect("Only Integral Number Available.");
    println!("Chk input : {}", num);

    let result = factorial(num);
    println!("{}! = {}", num, result);
}

fn factorial(n: i32) -> i32 {
    tail_fact(n, 1)
}

fn tail_fact(n: i32, result: i32) -> i32 {
    if n == 0 {
        result
    } else {
        tail_fact(n - 1, n * result)
    }
}
