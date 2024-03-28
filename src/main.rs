use database::{create_database, insert_data};
use handle_csv::{get_content, read_data};
use std::io::Error;
mod database;
mod handle_csv;
mod model;



fn main() -> Result<(), Box<Error>> {
    println!("start reading csv");
    create_database().expect("error on creating database");
    const FILENAME: &str = "./data.csv";
    let file = read_data(FILENAME).expect("Erro ao ler o arquivo");
    let content = get_content(file).expect("error on get content");
    insert_data(content).expect("error on save data in database");
    println!("finished save in local database");
    Ok(())
}



