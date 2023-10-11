use std::fs::File;
use std::io::prelude::*;

pub fn read_input(filename: &str) -> Result<String, std::io::Error> {
    // open file given as input and returns its content as a String
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
