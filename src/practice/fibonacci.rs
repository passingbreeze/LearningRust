use std::io;
use std::io::Write;

fn fib(n: u32) -> u32 {
    if n <= 1 {
        // 기본 사례입니다.
        n
    } else {
        // 재귀 사례입니다.
        fib(n - 1) + fib(n - 2)
    }
}

pub fn run() {
    let mut input = String::new();

    print!("Enter a number for Fibonacci >> ");
    // 출력을 플러시하여 입력 프롬프트가 즉시 나타나도록 합니다.
    io::stdout().flush().expect("Flush Failed!");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // 문자열을 정수로 파싱합니다.
    let n: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid non-negative number.");
            return;
        }
    };

    println!("fib({n}) = {}", fib(n));
}
