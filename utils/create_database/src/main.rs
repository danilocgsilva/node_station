use std::io::{self, Write};

fn ask_data(asking_title: &str) -> String {
    print!("{asking_title}");
    io::stdout().flush().unwrap();
    let mut raw_input: String = String::new();
    io::stdin().read_line(&mut raw_input).expect("error: unable to read user input.");
    String::from(raw_input.trim_end())
}

fn main() {
    let database_host: String = ask_data("Tell me the database host: ");
    let database_user: String = ask_data("Tell me the database user: ");
    let database_password: String = ask_data("Tell me the database password: ");
    let database_port: String = ask_data("Tell me the database port: ");

    println!("The database host is {database_host}.");
    println!("The database user is {database_user}.");
    println!("The database password is {database_password}.");
    println!("The database port is {database_port}.");
}
