use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use crate::read_lines;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point { row: usize, col: usize }
type Board = Vec<Vec<i64>>;

type Basin = Vec<Point>;

pub(crate) fn prob9() {
    let lines: Vec<String> = read_lines("./input/prob9.txt");
    let board: Board = parse_board(lines);
    println!("{}", part1(board.clone()));
    println!("{}", part2(board.clone()));
}

// Your first goal is to find the low points -
// the locations that are lower than any of its adjacent locations.
// Most locations have four adjacent locations (up, down, left, and right);
// locations on the edge or corner of the map have three or two adjacent locations, respectively.
// (Diagonal locations do not count as adjacent.)


// The risk level of a low point is 1 plus its height.
// What is the sum of the risk levels of all low points on your heightmap?
fn part1(board: Board) -> i64 {
    let mut res: i64 = 0;

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            let value = board[row][col];
            if is_low_point(row, col, board.clone()) {
                res = res + value + 1;
            }
        }
    }

    return res;
}


// A basin is all locations that eventually flow downward to a single low point.
// Therefore, every low point has a basin, although some basins are very small.

// Locations of height 9 do not count as being in any basin,
// and all other locations will always be part of exactly one basin.o

// The size of a basin is the number of locations within the basin, including the low point.

// Find the three largest basins and multiply their sizes together
fn part2(board: Board) -> i64 {
    let low_points: Vec<Point> = low_points(board.clone());
    println!("found {} low_points", low_points.clone().len());
    let mut basins: Vec<Basin> = low_points.into_iter().map(|low_point| get_basin(low_point, board.clone())).collect();
    println!("found {} basins", basins.clone().len());

    basins.sort_by(|b1, b2| if b1.len() < b2.len() { Ordering::Greater } else if b1.len() > b2.len() { Ordering::Less } else { Ordering::Equal});
    return (basins.get(0).map(|b| b.len()).unwrap_or(1) as i64)
     * (basins.get(1).map(|b| b.len()).unwrap_or(1) as i64)
     * (basins.get(2).map(|b| b.len()).unwrap_or(1) as i64);

}

fn get_basin(low_point: Point, board: Board) -> Basin {
    // breadth first search starting at the low point
    let mut to_visit: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    to_visit.push_back(low_point.clone());
    visited.insert(low_point.clone());

    while !to_visit.is_empty() {
        let cur: Point = to_visit.pop_front().unwrap();
        let cur_value: i64 = board[cur.row][cur.col];
        // Locations of height 9 do not count as being in any basin,

        let nbrs = neighbors(cur, board.clone());
        for nbr in nbrs {
            let nbr_value: i64 = board[nbr.row][nbr.col];
            if (!visited.contains(&nbr)) && nbr_value < 9 && nbr_value > cur_value {
                visited.insert(nbr.clone());
                to_visit.push_back(nbr.clone());
            }
        }
    }

    return visited.into_iter().collect();
}

fn neighbors(p: Point, board: Board) -> Vec<Point> {
    let above: Option<Point> = if p.row == 0 { None } else { Some(Point { row: p.row - 1, col: p.col }) };
    let below: Option<Point> = if p.row == (board.len() - 1) { None } else { Some(Point { row: p.row + 1, col: p.col }) };
    let left: Option<Point> = if p.col == 0 { None } else { Some(Point { row: p.row, col: p.col - 1 }) };
    let right: Option<Point> = if p.col == (board[0].len() - 1) { None } else { Some(Point { row: p.row, col: p.col + 1 }) };

    return vec![above, below, left, right].into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
}

fn low_points(board: Board) -> Vec<Point> {
    // breadth first search from each low point
    let mut low_points: Vec<Point> = Vec::new();

    for row in 0..board.len() {
        for col in 0..board[0].len() {
            if is_low_point(row, col, board.clone()) {
                low_points.push(Point { row, col } );
            }
        }
    }

    return low_points;
}

fn is_low_point(row: usize, col: usize, board: Board) -> bool {
    let above: i64 = if row == 0 { 1024} else { board[row - 1][col] };
    let below: i64 = if row == (board.len() - 1) { 1024 } else { board[row + 1][col] };
    let left: i64 = if col == 0 { 1024 } else { board[row][col - 1] };
    let right: i64 = if col == (board[0].len() - 1) { 1024 } else { board[row][col + 1] };
    let value: i64 = board[row][col];

    return value < above && value < below && value < left && value < right;
}

fn parse_board(lines: Vec<String>) -> Board {
    return lines.into_iter().map(|line| parse_line(line)).collect();
}

fn parse_line(line: String) -> Vec<i64> {
    return line.chars().into_iter().map(|ch| ch.to_string().parse::<i64>().unwrap()).collect();
}