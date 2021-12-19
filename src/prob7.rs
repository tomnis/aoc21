use std::cmp::min;
use crate::read_lines;

pub(crate) fn prob7() {
    let lines: Vec<String> = read_lines("./input/prob7.txt");
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));
}

// crab submarines can only move horizontally.
// Each change of 1 step in horizontal position of a single crab costs 1 fuel
// Determine the horizontal position that the crabs can align to using the least fuel possible.
// How much fuel must they spend to align to that position?
fn part1(lines: Vec<String>) -> i64 {
    let crabs: Vec<i64> = lines.clone()[0].split(",").map(|x| x.parse().unwrap()).collect();

    let min_pos: i64 = crabs.clone().into_iter().min().unwrap();
    let max_pos: i64 = crabs.clone().into_iter().max().unwrap();
    let scores: Vec<i64> = (min_pos..(max_pos + 1)).into_iter().map(|pos| score(pos, crabs.clone())).collect();
    return scores.into_iter().min().unwrap();
}

fn part2(lines: Vec<String>) -> i64 {
    let crabs: Vec<i64> = lines.clone()[0].split(",").map(|x| x.parse().unwrap()).collect();

    let min_pos: i64 = crabs.clone().into_iter().min().unwrap();
    let max_pos: i64 = crabs.clone().into_iter().max().unwrap();
    let scores: Vec<i64> = (min_pos..(max_pos + 1)).into_iter().map(|pos| score2(pos, crabs.clone())).collect();
    // let min_score = min(scores.clone());
    let min_score: i64 = scores.into_iter().min().unwrap();
    return min_score;
}

fn score(goal: i64, crabs: Vec<i64>) -> i64 {
    return crabs.into_iter().map(|c| (goal - c).abs()).sum();
}

// 99976804 too low
fn score2(goal: i64, crabs: Vec<i64>) -> i64 {
    return crabs.into_iter().map(|c| (1..((goal - c).abs() + 1)).sum::<i64>()).sum();
}