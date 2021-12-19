use std::borrow::Borrow;
use std::collections::{HashSet, VecDeque};
use crate::read_lines;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point { row: usize, col: usize }
type OctopusBoard = Vec<Vec<i64>>;

pub(crate) fn prob11() {
    let lines: Vec<String> = read_lines("./input/prob11.txt");
    let board: OctopusBoard = parse_board(lines.clone());
    println!("{}", part1(board.clone()));
    println!("{}", part2(board.clone()));
}


//  single step
// First, the energy level of each octopus increases by 1.
// Then, any octopus with an energy level greater than 9 flashes.
// This increases the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
// If this causes an octopus to have an energy level greater than 9, it also flashes.
// This process continues as long as new octopuses keep having their energy level increased beyond 9. (An octopus can only flash at most once per step.)
// Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.

// How many total flashes are there after 100 steps?
// 1007 too low
// 9823 too high
fn part1(mut board: OctopusBoard) -> i64 {
    let mut num_flashes: i64 = 0;

    for step in 0..100 {
        // First, the energy level of each octopus increases by 1.
        for row in 0..board.len() {
            for col in 0..board[row].len() {
                board[row][col] += 1;
            }
        }

        // Then, any octopus with an energy level greater than 9 flashes. these should be starting points for BFS
        let tf: Vec<Point> = greater_than(9, board.clone());
        println!("to flash points {}", tf.len());

        let (bd, flashed_pts) = bfs(tf, OctopusBoard::from(board.borrow()));
        num_flashes += flashed_pts.len() as i64;
        board = bd;
        // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
        for flashed in flashed_pts {
            board[flashed.row][flashed.col] = 0;
        }
    }

    return num_flashes;
}


// What is the first step during which all octopuses flash?
fn part2(mut board: OctopusBoard) -> i64 {
    let num_cells: usize = board.clone().into_iter().map(|row| row.len()).sum();
    let mut step: i64 = 0;
    let mut stop: bool = false;

    while !stop {
        step += 1;
        for row in 0..board.len() {
            for col in 0..board[row].len() {
                board[row][col] += 1;
            }
        }

        // any octopus with energy level greater than 9 flashes. starting points for BFS
        let first_flashed_octopuses: Vec<Point> = greater_than(9, board.clone());
        println!("to flash points {}", first_flashed_octopuses.len());

        let (bd, flashed_octopuses): (OctopusBoard, Vec<Point>) = bfs(first_flashed_octopuses, OctopusBoard::from(board.borrow()));
        board = bd;
        // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
        for flashed in flashed_octopuses.clone() {
            board[flashed.row][flashed.col] = 0;
        }

        stop = flashed_octopuses.clone().len() == num_cells as usize;
    }

    return step;
}



fn neighbors(p: Point, board: OctopusBoard) -> Vec<Point> {
    let above: Option<Point> = if p.row == 0 { None } else { Some(Point { row: p.row - 1, col: p.col }) };
    let above_left: Option<Point> = if p.row == 0 || p.col == 0 { None } else { Some(Point { row: p.row - 1, col: p.col - 1})};
    let above_right: Option<Point> = if p.row == 0 || p.col == (board[0].len() - 1) { None } else { Some(Point { row: p.row - 1, col: p.col + 1})};
    let below: Option<Point> = if p.row == (board.len() - 1) { None } else { Some(Point { row: p.row + 1, col: p.col }) };
    let below_left: Option<Point> = if p.row == (board.len() - 1) || p.col == 0 { None } else { Some(Point { row: p.row + 1, col: p.col - 1})};
    let below_right: Option<Point> = if p.row == (board.len() - 1) || p.col == (board[0].len() - 1) { None } else { Some(Point { row: p.row + 1, col: p.col + 1})};
    let left: Option<Point> = if p.col == 0 { None } else { Some(Point { row: p.row, col: p.col - 1 }) };
    let right: Option<Point> = if p.col == (board[0].len() - 1) { None } else { Some(Point { row: p.row, col: p.col + 1 }) };

    return vec![above, above_left, above_right, below, below_left, below_right, left, right].into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
}

// return points visited (points flashed)
fn bfs(mut starting_points: Vec<Point>, mut board: OctopusBoard) ->  (OctopusBoard, Vec<Point>) {
    let mut frontier: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();
    for pt in starting_points.into_iter() {
        frontier.push_back(pt.clone());
        visited.insert(pt.clone());
    }

    while !frontier.is_empty() {
        let cur: Point = frontier.pop_front().unwrap();
        let nbrs: Vec<Point> = neighbors(cur, board.clone());
        // increase the energy level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
        for nbr in nbrs {
            board[nbr.row][nbr.col] += 1;
            // If this causes an octopus to have an energy level greater than 9, it also flashes.
            if board[nbr.row][nbr.col] > 9 && !visited.contains(&nbr) {
                visited.insert(nbr.clone());
                frontier.push_back(nbr.clone());
            }
        }
    }

    return (board, visited.into_iter().collect());
}

// find all octopus that are about to flash
fn greater_than(target: i64, board: OctopusBoard) -> Vec<Point> {
    let mut res: Vec<Point> = Vec::new();

    for row in 0..board.len() {
        for col in 0..board[row].len() {
            if board[row][col] > target {
                res.push(Point { row, col });
            }
        }
    }

    return res;
}

fn parse_board(lines: Vec<String>) -> OctopusBoard {
    return lines.into_iter().map(|line| line.chars().map(|ch| ch.to_string().parse::<i64>().unwrap()).collect()).collect();
}

