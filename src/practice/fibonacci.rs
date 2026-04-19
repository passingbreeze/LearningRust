use std::convert::TryInto;
use super::helper;

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
    helper::flush_stdout("Enter a number for Fibonacci >> ");
    let n: Result<u32, _> = helper::read_number().try_into();

    match n {
        Ok(n) => {
            println!("fib({n}) = {}", fib(n));
        },
        Err(e) => {
            println!("음수를 넣어서 에러가 났을것 같은데... 자세한건 아래로");
            println!("{}", e.to_string());
        }
    }

}
