use rusqlite::{params, Connection, Result};
use std::error::Error;
use crate::model::Data;

pub fn create_database() -> Result<(), Box<dyn Error>> {

    let conn = Connection::open("./my_db.db3")?;
    conn.execute("CREATE TABLE IF NOT EXISTS data (
        id INTEGER PRIMARY KEY,
        description TEXT NOT NULL,
        price INTEGER NOT NULL
    )", params![])?;
    Ok(())

}

pub fn insert_data(data: Vec<Data>) -> Result<(), Box<dyn Error>> {

    let mut conn = Connection::open("./my_db.db3")?;
    let tx = conn.transaction()?;

    for item in data {
        tx.execute("INSERT INTO data (description, price) VALUES (?, ?)", 
        params![item.description, item.price])?;
    }

    tx.commit()?;

    Ok(())
}