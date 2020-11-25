use std::fs::File;
use std::io::{self, Read};
// use std::io::prelude::*;

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub async fn async_read_file(path: &str) -> io::Result<String>{
    let mut file1 = File::open(path)?;
    let mut buffer = String::new();
    file1.read_to_string(&mut buffer)?;

    Ok(buffer)
}