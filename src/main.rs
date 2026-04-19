mod practice;

use std::error::Error;
use practice::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("--- Running Practice Examples ---");
    
    // 주석을 해제하여 원하는 연습 예제를 실행할 수 있습니다.
    
    // println!("\n[1] Thread Test:");
    // thread_test::run();

    // println!("\n[2] File I/O:");
    // fileio::run();

    // println!("\n[3] Find Prime:");
    // findprime::run();

    // println!("\n[4] Guessing Game:");
    // guessing_game::run();

    // println!("\n[5] Tail Recursion:");
    // tail_recur::run();

    // 현재 기본값으로 thread_test를 실행합니다.
    // thread_test::run();

    // fibonacci::run();

    // shadowing::run();
    collatz::run();
    Ok(())
}
