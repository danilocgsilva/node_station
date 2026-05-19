use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("This will print every second.");
        thread::sleep(Duration::from_secs(1));
    }
}
