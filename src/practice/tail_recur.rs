use super::helper;

pub fn run() {
    helper::flush_stdout("Enter number >> ");
    let num = helper::read_number();
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
