use csv::ReaderBuilder;
use std::{error::Error, fs::File};
use crate::model::Data;


pub fn read_data(filename: &str) -> Result<File, Box<dyn Error>> {
    let result = File::open(filename)?;
    Ok(result)
}

pub fn get_content(file: File) -> Result<Vec<Data>, Box<dyn Error>> {
    let mut csv = ReaderBuilder::new().delimiter(b';').from_reader(file);
    let mut data = Vec::<Data>::new();

    for item in csv.deserialize::<Data>() {
        let record = item?;
        data.push(record);
    }
    Ok(data)
}