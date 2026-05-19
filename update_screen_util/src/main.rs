use std::thread;
use std::time::Duration;
mod update_screen;

fn main() {
    update_screen::exec("Danilo");
    thread::sleep(Duration::from_millis(1000));

    update_screen::exec("Silva");
    thread::sleep(Duration::from_millis(1000));

    update_screen::exec("Góes");
    thread::sleep(Duration::from_millis(1000));
}
