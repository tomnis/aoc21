#![feature(drain_filter)]
use std::fs::File;
use std::io::{BufReader, Lines};
use crate::read_lines;
use crate::util::char_at;

pub(crate) fn prob3() {
    let lns: Lines<BufReader<File>> = read_lines("./input/prob3.txt").unwrap();
    let lines: Vec<String> = lns.map(|x| x.unwrap()).collect();
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));
}

fn part1(lines: Vec<String>) -> i64 {
    // power consumption found by multiplying gamma rate by epsilon rate.
    return gamma(lines.clone()) * epsilon(lines.clone());
}


fn part2(lines: Vec<String>) -> i64 {
    // life support rating, which can be determined by multiplying the oxygen generator rating by the CO2 scrubber rating.
    let oxy = oxygen_rating(lines.clone());
    let co2 = co2_scrubber_rating(lines.clone());
    println!("oxy {}", oxy);
    println!("co2 {}", co2);
    return oxy * co2;
}

//
// Keep only numbers selected by the bit criteria for the type of rating value for which you are searching.
// Discard numbers which do not match the bit criteria.
// If you only have one number left, stop; this is the rating value for which you are searching.
// Otherwise, repeat the process, considering the next bit to the right.

fn oxygen_rating(lines: Vec<String>) -> i64 {
    lines.drain_filter(|x| x == "x".to_string());
    let mut lns: Vec<String> = lines.clone();
    let len:i32 = lns.get(0).unwrap().len() as i32;
    for col in 0..len {
        // There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position
        let mut crit: char = most_common(col, lns.clone());
        // If 0 and 1 are equally common, keep values with a 1 in the position being considered.
        if crit == '2' {
            crit = '1'
        }
        lns = lns.into_iter().filter(|x| char_at(x.to_string(), col) == crit).collect();
        println!("oxy len {}", lns.len());
        if lns.len() == 1 {
            return isize::from_str_radix(lns.get(0).unwrap(), 2).unwrap() as i64;
        }
    }
    return -1;
}

fn co2_scrubber_rating(lines: Vec<String>) -> i64 {
    let mut lns: Vec<String> = lines.clone();
    let len:i32 = lns.get(0).unwrap().len() as i32;

    for col in 0..len {
        // There are more 1 bits (7) than 0 bits (5), so keep only the 7 numbers with a 1 in the first position
        let mut crit: char = least_common(col, lns.clone());
        // If 0 and 1 are equally common, keep values with a 0 in the position being considered.
        if crit == '2' {
            crit = '0'
        }
        lns = lns.into_iter().filter(|x| char_at(x.to_string(), col) == crit).collect();
        println!("len {}", lns.len());
        if lns.len() == 1 {
            return isize::from_str_radix(lns.get(0).unwrap(), 2).unwrap() as i64;
        }
    }
    return -1;
}



// Each bit in the gamma rate can be determined by finding the most common bit
// in the corresponding position of all numbers in the diagnostic report.
fn gamma(lines: Vec<String>) -> i64 {
    let len:i32 = lines.get(0).unwrap().len() as i32;
    let mut res: String = "".to_string();
    for col in 0..len {
        let ch = most_common(col.clone(), lines.clone());
        res.push(ch);
    }
    println!("gamma: {}", res);
    let gamma= isize::from_str_radix(&*res, 2).unwrap();
    println!("gamma: {}", gamma);
    return gamma as i64;
}


// The epsilon rate is calculated in a similar way; rather than use the most common bit,
// the least common bit from each position is used.
// TODO could also be calculated as bitwise not of gamma
fn epsilon(lines: Vec<String>) -> i64 {
    let len:i32 = lines.get(0).unwrap().len() as i32;
    let mut res: String = "".to_string();
    for col in 0..len {
        let ch = least_common(col.clone(), lines.clone());
        res.push(ch);
    }
    println!("epsilon: {}", res);
    let epsilon = isize::from_str_radix(&*res, 2).unwrap();
    println!("epsilon: {}", epsilon);
    return epsilon as i64;
}


// TODO ugly and duplicated. is there some bitwise operation we can use?
fn most_common(col: i32, lines: Vec<String>) -> char {
    let mut num_zeros: i32 = 0;
    let mut num_ones: i32 = 0;
    for s in lines {
        let ch: char = char_at(s.clone(), col);
        if ch == '0' {
            num_zeros += 1;
        }
        else if ch == '1' {
            num_ones += 1;
        }
        else {
            panic!("not working {}", s);
        }
    }
    if num_ones > num_zeros {
        return '1';
    }
    else if num_zeros > num_ones {
        return '0';
    }
    else {
        // panic!("tied {}", col);
        // tied
        return '2';
    }
}


fn least_common(col: i32, lines: Vec<String>) -> char {
    let mut num_zeros: i32 = 0;
    let mut num_ones: i32 = 0;
    for s in lines {
        // let ch: char = s.chars().get(col as usize).unwrap().into();
        let ch: char = char_at(s.clone(), col);
        if ch == '0' {
            num_zeros += 1;
        }
        else if ch == '1' {
            num_ones += 1;
        }
        else {
            panic!("not working {}", s);
        }
    }
    if num_ones < num_zeros {
        return '1';
    }
    else if num_zeros < num_ones {
        return '0';
    }
    else {
        // panic!("tied {}", col);
        return '2';
    }
}