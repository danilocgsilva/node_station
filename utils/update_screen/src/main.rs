use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    for i in 0..10 {
        print!("\rRefreshing: {}%", i * 10);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(400));
    }

    print!("\r{}\r", " ".repeat(20));

    println!("Done!");
}

