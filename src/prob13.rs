use std::collections::HashSet;
use crate::read_lines;


// the coordinate 3,0 is to the right of 0,0, and the coordinate 0,7 is below 0,0.
// y==row, x==col
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point { x: usize, y: usize }

// should be either x or y
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Fold { axis: char, coord: usize }

type Input = (Vec<Point>, Vec<Fold>);

pub(crate) fn prob13() {
    let lines: Vec<String> = read_lines("./input/prob13.txt");
    let input = parse_lines(lines.clone());
    println!("{}", part1(input.clone()));
    println!("{}", part2(input.clone()));
}

fn part1(input: Input) -> i64 {
    let (points, folds): Input = input;

    // build points that are "#"
    let mut board: HashSet<Point> = HashSet::new();
    for p in points {
        board.insert(p);
    }

    let after_one_fold: HashSet<Point> = fold(board, folds.clone().into_iter().nth(0).unwrap());

    return after_one_fold.len() as i64;
}

fn part2(input: Input) -> String {
    let (points, folds): Input = input;

    // build points that are "#"
    let mut paper: HashSet<Point> = HashSet::new();
    for p in points {
        paper.insert(p);
    }


    for f in folds {
        paper = fold(paper, f);
    }
    // print output

    print_output(paper);

    return "".to_string();
}


fn print_output(paper: HashSet<Point>) -> () {
    let max_x: usize = paper.clone().into_iter().map(|p| p.x).max().unwrap();
    let max_y: usize = paper.clone().into_iter().map(|p| p.y).max().unwrap();

    for i in 0..max_y + 1 {
        for j in 0..max_x + 1 {
            let ch = if paper.contains(&Point { x: j, y: i }) { '#' } else { '.' };
            print!("{}", ch.to_string());
        }
        println!("");
    }
}


fn fold(paper: HashSet<Point>, fold: Fold) -> HashSet<Point> {
    if fold.axis == 'x' {
        return fold_x(paper, fold);
    }
    else if fold.axis == 'y' {
        return fold_y(paper, fold);
    }
    else {
        panic!("{} fold axis not known ", fold.axis);
    }
}

fn fold_x(paper: HashSet<Point>, fold: Fold) -> HashSet<Point> {
    println!("folding along y={}", fold.coord);
    // init values
    let mut res: HashSet<Point> = paper.clone().into_iter().filter(|p| p.x < fold.coord).collect();

    let to_update: HashSet<Point> = paper.clone().into_iter().filter(|p| p.x > fold.coord).collect();


    for p in to_update {
        let new_p: Point = Point { x: fold.coord - (p.x - fold.coord), y: p.y};
        println!("new p x={}, y={}", new_p.x, new_p.y);

        res.insert(new_p);
    }


    return res;
}

// old along y=7, which designates the line formed by all of the positions where y is 7 (marked here with -):
//
// ...#..#..#.
// ....#......
// ...........
// #..........
// ...#....#.#
// ...........
// ...........
// -----------
// ...........
// ...........
// .#....#.##.
fn fold_y(paper: HashSet<Point>, fold: Fold) -> HashSet<Point> {
    println!("folding along y={}", fold.coord);
    // init values
    let mut res: HashSet<Point> = paper.clone().into_iter().filter(|x| x.y < fold.coord).collect();

    let to_update: HashSet<Point> = paper.clone().into_iter().filter(|x| x.y > fold.coord).collect();


    for p in to_update {
        let new_p: Point = Point { x: p.x, y: fold.coord - (p.y - fold.coord)};
        println!("new p x={}, y={}", new_p.x, new_p.y);

        res.insert(new_p);
    }


    return res;
}

fn parse_lines(lines: Vec<String>) -> Input {
    let split: Vec<Vec<String>> = lines.split(|f| f.to_string().is_empty()).map(|l| l.to_vec()).collect();
    let points: Vec<Point> = split[0].clone().into_iter().map(|x| parse_point(x)).collect();
    let folds: Vec<Fold> = split[1].clone().into_iter().map(|x| parse_fold(x)).collect();

    return (points, folds)
}

fn parse_point(line: String) -> Point {
    let s: Vec<i64> = line.split(",").map(|x| x.parse().unwrap()).collect();
    return Point { x : s[0] as usize, y: s[1] as usize }
}

fn parse_fold(line: String) -> Fold {
    let s: Vec<String> = line[11..line.len()].split("=").map(|s| s.to_string()).collect();
    return Fold { axis: s[0].chars().nth(0).unwrap(), coord: s[1].parse::<usize>().unwrap() };
}