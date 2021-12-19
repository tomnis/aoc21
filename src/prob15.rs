use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::read_lines;

// 0,0 is top left
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Point { row: usize, col: usize }

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point
}



type Board = Vec<Vec<i64>>;


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            // .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn prob15() {
    let lines: Vec<String> = read_lines("./input/prob15.txt");
    let b = parse_board(lines);
    // println!("{}", part1(b.clone()));
    println!("{}", part2(b.clone()));
}

fn part1(board: Board) -> i64 {
    let mut cur_pos: Point = Point { row: 0, col: 0 };
    let goal: Point = Point { row: board.len() - 1, col: board[0].len() - 1};
    println!("origin score: {}", board[0][0]);
    return find_min_risk_value(cur_pos, goal, board,  &mut (HashMap::new()));
}


// 2874 too low
// 2935 too high (emitted 2941)
// 4965 too high
fn part2(board: Board) -> i64 {
    let mut new_board: Board = board.clone();
    // tile all the columns
    for r in 0..board.clone().len() {
        let og_row: Vec<i64> = board[r].clone();

        for i in 0..4 {
            let mut new_row: Vec<i64> = og_row.clone().into_iter().map(|e| ((e + i) % 9) + 1).collect();
            new_board[r].append(&mut new_row);
        }
    }

    // tile all the rows
    let orig_len = board.clone().len();
    for i in 0..4 {
        for r in 0..orig_len {
            let og_row: Vec<i64> = new_board[r].clone();
            let mut new_row: Vec<i64> = og_row.clone().into_iter().map(|e| ((e + i) % 9) + 1).collect();
            println!("new row len {} ", new_row.len());
            new_board.push(new_row)
        }
    }

    println!("expanded board: {} {}", new_board.len(), new_board[0].len());
    let mut cur_pos: Point = Point { row: 0, col: 0 };
    let goal: Point = Point { row: new_board.len() - 1, col: new_board[0].len() - 1};
    println!("origin score: {}", new_board[0][0]);
    let (dist, prev) = dijkstra(cur_pos, goal, new_board.clone());

    return dist[&goal] - new_board[cur_pos.row][cur_pos.col];
}


fn dijkstra(start: Point, goal: Point, graph: Board) -> (HashMap<Point, i64>, HashMap<Point, Point>) {
    let mut q: BinaryHeap<State> = BinaryHeap::new();
    // let mut q: HashSet<Point> = HashSet::new();
    let mut dist: HashMap<Point, i64> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();

    // dist[v] ← INFINITY
    //  prev[v] ← UNDEFINED
    for r in 0..graph.len() {
        for c in 0..graph[r].len() {
            let p: Point = Point { row: r, col: c };
            dist.insert(p.clone(), i64::MAX);
        }
    }

    dist.insert(start.clone(), graph[start.clone().row][start.clone().col]);
    q.push(State{ cost: 0, position: start });


    while !&q.is_empty() {
        // u ← vertex in Q with min dist[u]
        let u: Point = q.pop().unwrap().position; // q.borrow().into_iter().map(|p| (p.clone(), dist.get(&p).unwrap())).min_by_key(|p| *p.1).unwrap().0;
        // q.remove(&u);
        println!("q size {}", q.len());

        if u == goal {
            return (dist, prev);
        }

        let nbrs: Vec<Point> = neighbors(u, graph.clone());
        for v in nbrs {
            //  alt ← dist[u] + length(u, v)
            let alt = dist.get(&u).unwrap() + graph[v.row][v.col];
            if alt < *dist.get(&v).unwrap() {
                dist.insert(v, alt);
                prev.insert(v, u);
                q.push(State { cost: alt as usize, position: v});
            }
        }
    }

    return (dist, prev);
}

fn find_min_risk_value(cur_pos: Point, goal: Point, board: Board, mut cache: &mut HashMap<Point, i64>) -> i64 {
    // println!("finding risk for {} {}", cur_pos.row, cur_pos.col);
    // if cur_pos == goal
    if cur_pos == goal {
        let risk: i64 = board[goal.row][goal.col];
        cache.insert(cur_pos, risk);
        return risk;
    }
    else if cache.contains_key(&cur_pos) {
        return *cache.get(&cur_pos).unwrap();
    }
    else {
        let nbrs: Vec<Point> = neighbors(cur_pos.clone(), board.clone());
        // find min, +, add that to cache
        let min_nbr_risk: i64 = nbrs.into_iter().map(|nbr| find_min_risk_value(nbr, goal.clone(), board.clone(), &mut cache)).min().unwrap();
        let cur_risk: i64 = min_nbr_risk + board.clone()[cur_pos.clone().row][cur_pos.clone().col];
        cache.insert(cur_pos.clone(), cur_risk);

        // return min of other paths + cur risk value
        return cur_risk;
    }
}

fn neighbors(p: Point, b: Board) -> Vec<Point> {
    let up: Option<Point> = if p.row == 0 { None } else { Some(Point { row: p.row - 1, col: p.col}) };
    let right: Option<Point> = if p.col == b[0].len() - 1 { None } else { Some(Point { row: p.row, col: p.col + 1 }) };
    let down: Option<Point> = if p.row == b.len() - 1 { None } else { Some(Point { row: p.row + 1, col: p.col }) };
    let left: Option<Point> = if p.col == 0 { None } else { Some(Point { row: p.row, col: p.col - 1 }) };

    return vec![up, right, down, left].into_iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
}

fn parse_board(lines: Vec<String>) -> Board {
    return lines.into_iter().map(|l| parse_row(l)).collect();
}

fn parse_row(line: String) -> Vec<i64> {
    return line.chars().map(|c| c.to_string().parse().unwrap()).collect();
}