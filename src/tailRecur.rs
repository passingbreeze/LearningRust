use std::io;
use std::io::Write;

fn main(){
    let mut num : String = String::new();

    print!("Enter number >> ");
    io::stdout().flush().expect("Flush Failed!");

    io::stdin().read_line(&mut num).expect("Input Failure.");
    let num : i32 = num.trim().parse().expect("Only Integral Number Available.");
    println!("Chk input : {}", num);

    let result : i32 = factorial(num);
    println!("{}! = {}", num, result);
}

fn factorial(n : i32) -> i32 {
    tailFact(n,1)
}

fn tailFact(n : i32, result : i32) -> i32{
    if n == 0 {
        result
    }
    else {
        tailFact(n-1, n*result)
    }
}