use ansi_term::Color;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();

    println!("Hello world!");
    thread::sleep(Duration::from_secs(3));

    match write!(&mut stdout, "{}", Color::RGB(255, 0, 0).paint("\x1b[1A\x1b[K")) {
        Ok(_) => {
            thread::sleep(Duration::from_secs(3));
            println!("Done with success")
        },
        Err(err) => eprintln!("Error: {}", err),
    }

    stdout.flush().unwrap();
}
