use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub(crate) fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}


pub(crate) fn char_at(s: String, i: i32) -> char {
    let j: usize = i as usize;
    return s.get(j..j+1).unwrap().chars().collect::<Vec<char>>()[0];
}