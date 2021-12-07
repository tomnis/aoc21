#![feature(iter_zip)]
use std::cmp::{max, min};
use std::str::Split;
use std::collections::HashMap;
use crate::read_lines;

// x1,y1 are the coordinates of one end the line segment and x2,y2 are the coordinates of the other end.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point { x: i64, y: i64 }
#[derive(Clone, Debug)]
struct LineSegment { start: Point, end: Point }


pub(crate) fn prob5() {
    let lines: Vec<String> = read_lines("./input/prob5.txt");
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));
}


// the number of points where at least two lines overlap
fn part1(lines: Vec<String>) -> i64 {
    let segments: Vec<LineSegment> = parse_line_segments(lines);
    println!("parsed {} segments", segments.len());
    // build a hashmap of all points

    // get horizontal and vertical lines
    let horz_and_vert_segments: Vec<LineSegment> = segments.clone().into_iter().filter(|s| is_horizontal_or_vertical(s.clone())).collect();

    // given a line segment, get a list of all points on that segment
    let points_on_segments: Vec<Vec<Point>> = horz_and_vert_segments.into_iter().map(|seg| points_on_segment(seg)).collect();

    // iterate over points, incrementing hashmap count
    let mut pts: HashMap<Point, i64> = HashMap::new();
    for ln in points_on_segments {
        for point in ln {
            if !pts.contains_key(&point) {
                pts.insert(point, 1);
            }
            else {
                let old: i64 = *(pts.get(&point).unwrap());
                let new: i64 = old + 1;
                pts.insert(point, new);
            }
        }
    }

    return pts.into_iter().filter(|(pt, cnt)| *cnt >= 2).collect::<Vec<(Point, i64)>>().len() as i64;
}


fn part2(lines: Vec<String>) -> i64 {

    let segments: Vec<LineSegment> = parse_line_segments(lines);
    println!("parsed {} segments", segments.len());
    // given a line segment, get a list of all points on that segment
    let points_on_segments: Vec<Vec<Point>> = segments.into_iter().map(|seg| points_on_segment(seg)).collect();
    // iterate over points, incrementing hashmap count
    let mut pts: HashMap<Point, i64> = HashMap::new();
    for ln in points_on_segments {
        for point in ln {
            if !pts.contains_key(&point) {
                pts.insert(point, 1);
            }
            else {
                let old: i64 = *(pts.get(&point).unwrap());
                let new: i64 = old + 1;
                pts.insert(point, new);
            }
        }
    }

    return pts.into_iter().filter(|(pt, cnt)| *cnt >= 2).collect::<Vec<(Point, i64)>>().len() as i64;
}


fn parse_line_segments(lines: Vec<String>) -> Vec<LineSegment> {
    let mut res: Vec<LineSegment> = Vec::new();

    for line in lines {
        let points: Split<&str> = line.split(" -> ");
        let startstr: String = points.clone().into_iter().nth(0).unwrap().to_string();
        let endstr: String = points.clone().into_iter().nth(1).unwrap().to_string();

        let x1y1: Vec<i64> = startstr.split(",").map(|s| s.parse().unwrap()).collect();
        let x2y2: Vec<i64> = endstr.split(",").map(|s| s.parse().unwrap()).collect();

        let segment: LineSegment = LineSegment { start: Point { x: x1y1[0], y: x1y1[1]}, end: Point { x: x2y2[0], y: x2y2[1]} };
        res.push(segment)
    }

    return res;
}

fn points_on_segment(segment: LineSegment) -> Vec<Point> {
    // vertical
    if segment.start.x == segment.end.x {
        let min_y: i64 = min(segment.start.y, segment.end.y);
        let max_y: i64 = max(segment.start.y, segment.end.y);

        return (min_y..(max_y + 1)).into_iter().map(|y| Point {x: segment.start.x, y: y}).collect();
    }
        // horz
    else if segment.start.y == segment.end.y {
        let min_x: i64 = min(segment.start.x, segment.end.x);
        let max_x: i64 = max(segment.start.x, segment.end.x);

        return (min_x..(max_x + 1)).into_iter().map(|x| Point {x: x, y: segment.start.y}).collect();
    }
        // only 45 degree angles
    else if segment.start.x < segment.end.x && segment.start.y < segment.end.y {
        let xs: Vec<i64> = (segment.start.x..(segment.end.x + 1)).collect();
        let ys: Vec<i64> = (segment.start.y..(segment.end.y + 1)).collect();
        if xs.len() != ys.len() {
            panic!("unequal lengths")
        }

        return xs.into_iter().zip(ys.into_iter()).map(|(x, y)| Point { x: x, y: y }).collect();
    }
    else if segment.start.x < segment.end.x && segment.start.y > segment.end.y {
        let xs: Vec<i64> = (segment.start.x..(segment.end.x + 1)).collect();
        let mut ys: Vec<i64> = (segment.end.y..(segment.start.y + 1)).collect();
        ys.reverse();
        if xs.len() != ys.len() {
            panic!("unequal lengths")
        }

        return xs.into_iter().zip(ys.into_iter()).map(|(x, y)| Point { x: x, y: y }).collect();
    }
    else if segment.start.x > segment.end.x && segment.start.y < segment.end.y {
        // need to decrement x
        let mut xs: Vec<i64> = (segment.end.x..(segment.start.x + 1)).collect();
        xs.reverse();
        let ys: Vec<i64> = (segment.start.y..(segment.end.y + 1)).collect();
        if xs.len() != ys.len() {
            panic!("unequal lengths {} {}", xs.len(), ys.len())
        }

        return xs.into_iter().zip(ys.into_iter()).map(|(x, y)| Point { x: x, y: y }).collect();
    }
    else if segment.start.x > segment.end.x && segment.start.y > segment.end.y {
        let xs: Vec<i64> = (segment.end.x..(segment.start.x + 1)).collect();
        let ys: Vec<i64> = (segment.end.y..(segment.start.y + 1)).collect();
        if xs.len() != ys.len() {
            panic!("unequal lengths {} {}", xs.len(), ys.len())
        }

        return xs.into_iter().zip(ys.into_iter()).map(|(x, y)| Point { x: x, y: y }).collect();
    }
    else {
        panic!("{} is not horizontal or vertical", segment.start.x);
    }
}

// horizontal and vertical lines: lines where either x1 = x2 or y1 = y2.
fn is_horizontal_or_vertical(segment: LineSegment) -> bool {
    return segment.start.x == segment.end.x || segment.start.y == segment.end.y;
}