use std::fs::File;
use std::io::{BufReader, Lines};
use crate::read_lines;

#[derive(Clone, Debug)]
struct BingoCell { num: i32, is_marked: bool}
type BingoBoard = Vec<Vec<BingoCell>>;

pub(crate) fn prob4() {
    let lines: Vec<String> = read_lines("./input/prob4.txt");
    // println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));
}

// The score of the winning board can now be calculated.
// Start by finding the sum of all unmarked numbers on that board; in this case, the sum is 188.
// Then, multiply that sum by the number that was just called when the board won, 24, to get the final score, 188 * 24 = 4512.


// To guarantee victory against the giant squid, figure out which board will win first.
// What will your final score be if you choose that board?

fn part1(lines: Vec<String>) -> i64 {
    // first line is a sequence of numbers to call out
    let call_nums: Vec<i32> = called_nums(lines.clone()[0].clone());
    let mut boards: Vec<BingoBoard> = parse_boards(lines.clone()[2..].to_vec());
    println!("parsed {} boards", boards.len());

    for num in call_nums {
        // mark the number
        boards = call_num(num, boards);

        // check if theres a winning board
        let maybe_winning_board: Option<BingoBoard> = winning_board(boards.clone());

        if maybe_winning_board.is_some() {
            let b = maybe_winning_board.unwrap();
            return score(b) * num as i64;
        }
    }
    // return 0;
    return -1;
}


// figure out which board will win last
fn part2(lines: Vec<String>) -> i64 {
    // first line is a sequence of numbers to call out
    let call_nums: Vec<i32> = called_nums(lines.clone()[0].clone());
    let mut boards: Vec<BingoBoard> = parse_boards(lines.clone()[2..].to_vec());
    println!("parsed {} boards", boards.len());

    for num in call_nums {
        // mark the number
        boards = call_num(num, boards.clone());

        let winning_boards: Vec<BingoBoard> = boards.clone().into_iter().filter(|b| is_winning(b.to_vec())).collect();
        for winning_board in winning_boards {
            println!("board won, score = {}", score(winning_board) * num as i64);
        }
        boards = boards.clone().into_iter().filter(|b| !is_winning(b.to_vec())).collect();
    }
    // return 0;
    return -1;
}


fn call_num(num: i32, boards: Vec<BingoBoard>) -> Vec<BingoBoard> {
    // println!("calling {}", num);
    return boards.into_iter().map(|b| call_num_board(num, b)).collect();
}

fn call_num_board(num: i32, board: BingoBoard) -> BingoBoard {
    let mut b: BingoBoard = board.clone();

    for i  in 0..b.len() {
        for j in 0..b[i].len() {
            if b[i][j].num == num {
                // println!("matching num {}", num);
                b[i][j].is_marked = true;
            }
        }
    }

    return b;
}

fn called_nums(line: String) -> Vec<i32> {
    return line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
}

fn parse_boards(lines: Vec<String>) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::new();

    println!("got {} board input lines", lines.len());

    let board_lines: Vec<String> = lines.into_iter().filter(|x| x.len() > 1).collect();
    println!("got {} board input lines (filtered out spacer lines)", board_lines.len());
    for bd_lines in board_lines.chunks(5) {
        let mut b: BingoBoard = parse_board(bd_lines.to_vec());
        boards.push(b);
    }

    println!("parsed {} boards", boards.len());
    return boards;
}


fn parse_board(lines: Vec<String>) -> BingoBoard {
    if lines.len() != 5 {
        panic!("not working got {} line chunk", lines.len());
    }

    let mut board: BingoBoard = Vec::new();

    for line in lines {
        // println!("line: {}", line);
        // let f = line.trim().split(" ");
        // for s in f {
        //     println!("s {}", s);
        // }
        let cells: Vec<BingoCell> = line.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).map(|x| BingoCell { num: x, is_marked: false }).collect();
        board.push(cells);
    }

    return board;
}

fn winning_board(boards: Vec<BingoBoard>) -> Option<BingoBoard> {
    return boards.clone().into_iter().find(|b| is_winning(b.clone()));
}

fn winning_board_and_idx(boards: Vec<BingoBoard>) -> Option<(usize, BingoBoard)> {
    return boards.clone().into_iter().enumerate().find(|(u,b)| is_winning(b.clone()));
}

fn is_winning(board: BingoBoard) -> bool {
    // check rows
    for row in board.clone() {
        if row.iter().filter(|x| x.is_marked).count() == row.len() {
            return true;
        }
    }

    // check cols
    for col in 0..board.clone()[0].len() {
        if is_col_winning(col as i32, board.clone()) {
            return true
        }
    }

    return false;
}


fn is_col_winning(col: i32, board: BingoBoard) -> bool {
    let mut cnt: i32 = 0;
    for row in board.clone() {
        if row[col as usize].is_marked {
            cnt += 1;
        }
    }

    return cnt == (board.clone().len() as i32);
}


// sum of all unmarked numbers
fn score(board: BingoBoard) -> i64 {
    let mut res: i64 = 0;
    for row in board {
        for cell in row {
            if !cell.is_marked {
                res += cell.num as i64;
            }
        }
    }
    return res;
}