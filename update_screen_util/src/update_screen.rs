use std::io::{stdout, Write};

pub fn exec(message: &str) {
    print!("\r{}", " ".repeat(20));
    print!("\r{}", message);
    stdout().flush().unwrap();
}