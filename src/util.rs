use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Result};
use std::path::Path;
// use im:Vector;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub(crate) fn read_lines<P>(filename: P) -> Vec<String> where P: AsRef<Path>, {
    let file = File::open(filename);
    return BufReader::new(file.unwrap()).lines().map(|line| line.unwrap()).collect();
}


pub(crate) fn char_at(s: String, i: i32) -> char {
    let j: usize = i as usize;
    return s.get(j..j+1).unwrap().chars().collect::<Vec<char>>()[0];
}


pub(crate) fn parse_binary(bits: String) -> usize {
    usize::from_str_radix(&*bits, 2).unwrap()
}


pub(crate) fn parse_binary_substring(bits: String, start: usize, end: usize) -> usize {
    let substr: String = bits[start..end].to_string();
    return parse_binary(substr);
}

// is a superset of b
pub(crate) fn is_superset(a: String, b: String) -> bool {
    return b.chars().all(|ch| a.contains(ch));
}

pub(crate) fn all_same_chars(a: String, b: String) -> bool {
    return is_superset(a.clone(), b.clone()) && is_superset(b.clone(), a.clone());
}