use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::read_lines;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Edge { a: String, b: String }

type Path = Vec<String>;
type Graph = HashMap<String, HashSet<String>>;

pub(crate) fn prob12() {
    let lines: Vec<String> = read_lines("./input/prob12.txt");
    let graph: Graph = edges2graph((parse_edges(lines)));
    println!("{}", part1(graph.clone()));
    println!("{}", part2(graph.clone()));
}

fn part1(graph: Graph) -> i64 {
    // So, all paths you find should visit small caves at most once, and can visit big caves any number of times.
    // How many paths through this cave system are there that visit small caves at most once?
    // find the number of distinct paths that start at start, end at end, and don't visit small caves more than once.
    let mut paths: VecDeque<Path> = VecDeque::new();
    let mut cnt: i64 = 0;

    paths.push_back(vec!["start".to_string()]);

    while !paths.is_empty() {
        let p = paths.pop_front().unwrap();
        if p.last().unwrap() == &"end".to_string() {
            cnt += 1
        }

        let nbrs: Vec<String> = graph.get(p.last().unwrap()).unwrap().into_iter().map(|s| s.to_string()).collect();
        for nbr in nbrs {
            if nbr.chars().nth(0).unwrap().is_uppercase() || !p.contains(&nbr) {
                let mut new_path = p.clone();
                new_path.push(nbr.clone());
                paths.push_back(new_path);
            }
        }
    }


    return cnt;
}


fn part2(graph: Graph) -> i64 {
    // So, all paths you find should visit small caves at most once, and can visit big caves any number of times.
    // How many paths through this cave system are there that visit small caves at most once?
    // find the number of distinct paths that start at start, end at end, and don't visit small caves more than once.
    let mut paths: VecDeque<Path> = VecDeque::new();
    let mut cnt: i64 = 0;

    paths.push_back(vec!["start".to_string()]);

    // you might have time to visit a single small cave twice
    while !paths.is_empty() {
        let p = paths.pop_front().unwrap();
        if p.last().unwrap() == &"end".to_string() {
            cnt += 1
        }

        let nbrs: Vec<String> = graph.get(p.last().unwrap()).unwrap().into_iter().map(|s| s.to_string()).collect();
        for nbr in nbrs {
            if nbr.chars().nth(0).unwrap().is_uppercase() || !p.clone().contains(&nbr) || can_visit_lower(nbr.clone(), p.clone()){
                let mut new_path = p.clone();
                new_path.push(nbr.clone());
                paths.push_back(new_path);
            }
        }
    }


    return cnt;
}

fn can_visit_lower(node: String, path: Vec<String>) -> bool {
    return node != "start".to_string() && node != "end".to_string() && lower_counts(path);
}


fn lower_counts(path: Vec<String>) -> bool {
    let mut seen: HashSet<String> = HashSet::new();

    for node in path {
        if node.chars().nth(0).unwrap().is_lowercase() {
            if seen.contains(&*node) {
                return false;
            }
            else {
                seen.insert(node.clone());
            }

        }
    }

    return true;
}

fn edges2graph(edges: Vec<Edge>) -> Graph {
    let mut g: Graph = HashMap::new();

    for edge in edges {
        if !g.contains_key(&*edge.a) {
            g.insert(edge.a.clone(), HashSet::new());
        }

        g.get_mut(&*(edge.a)).unwrap().insert(edge.clone().b);

        if !g.contains_key(&*(edge.clone().b)) {
            g.insert(edge.b.clone(), HashSet::new());
        }

        g.get_mut(&*(edge.b)).unwrap().insert(edge.a);
    }


    return g
}


fn parse_edges(lines: Vec<String>) -> Vec<Edge> {
    lines.into_iter().map(|line| parse_edge(line)).collect()
}

fn parse_edge(line: String) -> Edge {
    let s: Vec<String> = line.split("-").map(|str| str.to_string()).collect();
    let a: String = s[0].to_string();
    let b: String = s[1].to_string();

    return Edge { a, b };
}