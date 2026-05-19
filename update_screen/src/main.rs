// use std::io::{stdout, Write};
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let mut stdout = stdout();
//     for i in 0..10 {
//         print!("\rRefreshing: {}%", i * 10);
//         stdout.flush().unwrap();
//         thread::sleep(Duration::from_millis(400));
//     }
//     print!("\r\033[K");
//     stdout.flush().unwrap();
//     println!("Done!");
// }



use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
// use ansi_term::Style;

fn main() {
    let mut stdout = stdout();
    for i in 0..10 {
        print!("\rRefreshing: {}%", i * 10);
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(400));
    }

    // Clear the line before printing "Done!"
    // Style::new()
    //     .paint("\r\033[K")
    //     .write_to(&mut stdout).unwrap();

    // write!(&mut stdout, "\r\033[K").unwrap();

    // let clear_line = Style::new().paint("\r\033[K").to_string();
    // write!(&mut stdout, "{}", clear_line).unwrap();
    print!("\r{}\r", " ".repeat(20));

    println!("Done!");
}

