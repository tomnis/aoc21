use std::fs::File;
use std::io::{BufReader, Lines};
use crate::read_lines;


pub(crate) fn prob2() {
    let lns: Lines<BufReader<File>> = read_lines("./input/prob2.txt").unwrap();
    let lines: Vec<String> = lns.map(|x| x.unwrap()).collect();
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));


}

fn part1(lines: Vec<String>) -> i32 {
    //What do you get if you multiply your final horizontal position by your final depth?
    let mut horz: i32 = 0;
    let mut depth: i32 = 0;

    for cmd in lines {
        // println!("{} ", cmd);
        if cmd.starts_with("forward") {
            let hor: i32 = cmd[8..].parse::<i32>().unwrap();
            horz += hor
        }
        else if cmd.starts_with("up") {
            let u: i32 = cmd[3..].parse::<i32>().unwrap();
            depth -= u;
        }
        else if cmd.starts_with(("down")) {
            let u: i32 = cmd[5..].parse::<i32>().unwrap();
            depth += u;
        }
        else {
            panic!("not working {}", cmd);
        }
    }

    return horz * depth;
}


fn part2(lines: Vec<String>) -> i32 {
    let mut horz: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for cmd in lines {
        // println!("{} ", cmd);

        // forward X does two things:
        //     It increases your horizontal position by X units.
        //     It increases your depth by your aim multiplied by X.
        if cmd.starts_with("forward") {
            let x: i32 = cmd[8..].parse::<i32>().unwrap();
            horz += x;
            depth += (aim * x);
        }
        // up X decreases your aim by X units.
        else if cmd.starts_with("up") {
            let x: i32 = cmd[3..].parse::<i32>().unwrap();
            aim -= x;
        }
        // down X increases your aim by X units.
        else if cmd.starts_with(("down")) {
            let x: i32 = cmd[5..].parse::<i32>().unwrap();
            aim += x;
        }
        else {
            panic!("not working {}", cmd);
        }
    }

    return horz * depth;

}