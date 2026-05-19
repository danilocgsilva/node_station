use std::process;

fn main() {
    let cmd = "tput setaf 2";
    match process::Command::new("sh").arg("-c").arg(cmd).output() {
        Ok(_) => println!("Terminal info update successfuly!"),
        Err(err) => eprintln!("Error updating terminal info: {}", err),
    }
}
