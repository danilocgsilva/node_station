use std::io::{self, Write};
use tokio_postgres::{Client, NoTls};
use std::error::Error;

fn ask_data(asking_title: &str) -> String {
    print!("{asking_title}");
    io::stdout().flush().unwrap();
    let mut raw_input: String = String::new();
    io::stdin().read_line(&mut raw_input).expect("error: unable to read user input.");
    String::from(raw_input.trim_end())
}

fn ask_sensitive_data(asking_title: &str) -> String {
    rpassword::prompt_password(asking_title).expect("Error: unable to read password")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_host: String
        = ask_data("Tell me the database host: ");

    let database_user: String
        = ask_data("Tell me the database user: ");

    let database_password: String
        = ask_sensitive_data("Tell me the database password: ");

    let database_port: String
        = ask_data("Tell me the database port: ");

    println!("The database host is {database_host}.");
    println!("The database user is {database_user}.");
    println!("The database password is {database_password}.");
    println!("The database port is {database_port}.");

    let connection_string = format!(
        "host={} user={} password={} port={}",
        database_host, database_user, database_password, database_port
    );

    let (_client, _connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    // client.execute("CREATE DATABASE my_new_database", &[]).await?;

    Ok(())

    // let (client, connection) = Client::connect("host={database_host} user={database_user} password={database_password}, NoTls").await?;
    // Client::connect("host={database_host} user={database_user} password={database_password}, NoTls").await?
}
