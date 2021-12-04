use std::fs::File;
use std::io::{BufReader, Lines};
use std::slice::Windows;
use crate::read_lines;

pub(crate) fn prob1() {
    let lns: Lines<BufReader<File>> = read_lines("./input/prob1.txt").unwrap();
    let nums: Vec<i32> = lns.map(|x| x.unwrap().parse().unwrap()).collect();
    println!("{}", part1(nums.clone()));
    println!("{}", part2(nums.clone()));
}

fn part1(nums: Vec<i32>) -> i32 {
    return count_pairs_greater(nums);
}

fn part2(nums: Vec<i32>) -> i32 {
    let windows: Windows<i32> = nums.windows(3);
    // sum each window
    let summed: Vec<i32> = windows.map(|x| x.iter().sum::<i32>()).collect();
    return count_pairs_greater(summed);
}



fn count_pairs_greater(nums: Vec<i32>) -> i32 {
    let windows: Windows<i32> = nums.windows(2);
    let mut result: i32 = 0;
    for window in windows {
        let a: &i32 = window.get(0).unwrap();
        let b: &i32 = window.get(1).unwrap();
        if a < b {
            result = result + 1;
        }
    }
    return result
}