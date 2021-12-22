use std::borrow::Borrow;
use std::collections::VecDeque;
use crate::read_lines;

use either::Either;
use crate::util::char_at;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pair {
    a: Either<i64, Box<Pair>>,
    b: Either<i64, Box<Pair>>
}

pub(crate) fn prob18() {
    let lines: Vec<String> = read_lines("./input/prob18.txt");
    println!("{}", find_top_comma_idx("[[5,[7,4]],7],1".to_string()));
    println!("{}", find_top_comma_idx("1,1".to_string()));
    let pairs: Vec<Pair> = lines.into_iter().map(|line| parse_pair(line)).collect();
    println!("parsed {} snailfish numbers", pairs.len());
    println!("{}", part1(pairs.clone()));
}


fn part1(pairs: Vec<Pair>) -> i64 {
    let sum: Box<Pair> = sum_all_pairs(pairs);
    return magnitude(sum)
}


fn sum_all_pairs(pairs: Vec<Pair>) -> Box<Pair> {
    return Box::new(pairs.into_iter().reduce(|a,b|add_pairs(a, b)).unwrap());
}


// To add two snailfish numbers, form a pair from the left and right parameters of the addition operator.
// For example, [1,2] + [[3,4],5] becomes [[1,2],[[3,4],5]].
fn add_pairs(p1: Pair, p2: Pair) -> Pair {
    let sum = Pair { a: Either::Right(Box::new(p1)), b: Either::Right(Box::new(p2)) };
    return reduce(sum);
}



// repeatedly do the first action in this list that applies to the snailfish number:
//
//     If any pair is nested inside four pairs, the leftmost such pair explodes.
//     If any regular number is 10 or greater, the leftmost such regular number splits.
fn reduce(p: Pair) -> Pair {
    return Pair { a: Either::Left(0), b: Either::Left(0) };
}


// The magnitude of a pair is 3 times the magnitude of its left element plus 2 times the magnitude of its right element.
// The magnitude of a regular number is just that number
fn magnitude(p: Box<Pair>) -> i64 {
    3 * magnitude_e(p.a) + 2 * magnitude_e(p.b)
}

fn magnitude_e(e: Either<i64, Box<Pair>>) -> i64 {
    match e {
        Either::Left(l) => l,
        Either::Right(r) => magnitude(r)
    }
}

fn parse_pair(line: String) -> Pair {
    // strip off the outer layer of brackets
    let str: String = line[1..line.len()-1].to_string();

    let mid_comma_idx: usize = find_top_comma_idx(str.clone());

    let astr: String = str[0..mid_comma_idx].to_string();
    let a: Either<i64, Box<Pair>> = parse_element(astr);

    let bstr: String = str[mid_comma_idx+1..].to_string();
    let b: Either<i64, Box<Pair>> = parse_element(bstr);

    return Pair { a, b };
}

fn parse_element(e: String) -> Either<i64, Box<Pair>> {
    if e.len() == 1 {
        let n: i64 = e.parse().unwrap();
        return Either::Left(n);
    }
    else {
        return Either::Right(Box::new(parse_pair(e)));
    }
}

fn find_top_comma_idx(s: String) -> usize {
    if char_at(s.clone(), 0) != '[' {
        // println!("{}", s.clone());
        return 1;
    }
    else {
        let mut levels: i64 = 0;

        for i in 0..s.clone().len() {
            if char_at(s.clone(), i) == '[' {
                levels += 1;
            }
            else if char_at(s.clone(), i) == ']' {
                levels -= 1;
            }

            if levels == 0 {
                return i + 1;
            }
        }

        return 0;
    }
}