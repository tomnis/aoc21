use std::borrow::{Borrow, BorrowMut};
use std::cmp::max;
use crate::read_lines;


#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Pair2 { x: i64, y: i64 }
type Velocity = Pair2;
type Position = Pair2;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct TargetArea {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64
}

pub(crate) fn prob17() {
    let lines: Vec<String> = read_lines("./input/prob17.txt");
    let first_line: String = lines.clone().into_iter().nth(0).unwrap();
    // sample input
    // x=20..30, y=-10..-5
    let sample_area: TargetArea = TargetArea { xmin: 20, xmax: 30, ymin: -10, ymax: -5 };
    println!("{}", part1(sample_area));

    // target area: x=137..171, y=-98..-73
    let area: TargetArea = TargetArea { xmin: 137, xmax: 171, ymin: -98, ymax: -73 };
    println!("{}", part1(area.clone()));
    println!("{}", part2(area.clone()));
}

// In this diagram, S is the probe's initial position, 0,0.
// The x coordinate increases to the right, and the y coordinate increases upward.
// What is the highest y position it reaches on this trajectory?
fn part1(area: TargetArea) -> i64 {
    let init_pos: Position = Position { x: 0, y: 0};

    let mut max_y: i64 = 0;
    // let init_v: Velocity = Velocity{ x: 6,y: 9};
    for x_v in 1..100 {
        for y_v in 1..100 {
            let init_v: Velocity = Velocity{ x: x_v, y: y_v};
            let max_y_on_path: Option<i64> = shoot(init_pos, init_v, area);
            // shot didnt miss
            if max_y_on_path.is_some() {
                max_y = max(max_y, max_y_on_path.unwrap());
            }
        }
    }
    // Find the initial velocity that causes the probe to reach the highest y position and still eventually be within the target area after any step.
    // What is the highest y position it reaches on this trajectory?
    return max_y;
}


fn part2(area: TargetArea) -> i64 {
    let init_pos: Position = Position { x: 0, y: 0};
    let mut count: i64 = 0;


    // 636 too low
    for x_v in 1..1000 {
        for y_v in -1000..1000 {
            let init_v: Velocity = Velocity{ x: x_v, y: y_v};
            let max_y_on_path: Option<i64> = shoot(init_pos, init_v, area);
            // shot didnt miss
            if max_y_on_path.is_some() {
                count += 1;
            }
        }
    }

    return count;
}



// return max y position seen
fn shoot(init_pos: Position, init_v: Velocity, area: TargetArea) -> Option<i64> {
    let mut max_y = init_pos.y;

    let mut pos: Position = init_pos;
    let mut v: Velocity = init_v;

    loop {
        // println!("x: {}, y: {}", pos.x, pos.y);
        step(&mut pos, &mut v);
        max_y = max(max_y, pos.borrow().y);
        if is_within_area(pos.clone(), area.clone()) {
            return Some(max_y);
        } else if missed(pos.clone(), area.clone()) {
            return None;
        }
    }
}


// The probe's x position increases by its x velocity.
// The probe's y position increases by its y velocity.
// Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
// Due to gravity, the probe's y velocity decreases by 1.
fn step(pos: &mut Position, v: &mut Velocity) -> () {
    pos.x += v.x;
    pos.y += v.y;
    if v.x < 0 {
        v.x += 1;
    }
    else if v.x > 0 {
        v.x -= 1;
    }
    v.y -= 1;
}

// uccessfully make it into the trench, the probe must be on some trajectory that causes it to be within a target area after any step.
// The submarine computer has already calculated this target area (your puzzle input). For example
fn is_within_area(pos: Position, area: TargetArea) -> bool {
    return area.xmin <= pos.x && pos.x <= area.xmax && area.ymin <= pos.y && pos.y <= area.ymax;
}

fn missed(pos: Position, area: TargetArea) -> bool {
    return pos.y < area.ymin;
}