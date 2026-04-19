use std::io::{self, Write};

pub fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid integer")
}

pub fn flush_stdout(prompt: &str) {
    print!("{}", prompt);
    io::stdout().flush().expect("Flush failed");
}
