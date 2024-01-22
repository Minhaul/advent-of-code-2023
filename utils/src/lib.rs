use std::fs;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

pub fn file_to_vec(filename: &str) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);

    file_reader.lines().collect()
}
