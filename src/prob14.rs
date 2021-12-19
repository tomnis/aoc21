use std::collections::{HashMap, HashSet};
use crate::read_lines;

use std::slice::Windows;
use std::collections::LinkedList;
use std::hash::Hash;

type Polymer = String;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct PairInsertionRule { left: char, right: char, insert: char }

type Input = (Polymer, Vec<PairInsertionRule>);

pub(crate) fn prob14() {
    let lines: Vec<String> = read_lines("./input/prob14.txt");
    let input: Input = parse_input(lines.clone());
    // println!("{}", part1(input));
    println!("{}", part2(input.clone()));
}

// Apply 10 steps of pair insertion to the polymer template and find the most and least common elements in the result.
// What do you get if you take the quantity of the most common element and subtract the quantity of the least common element?
fn part1(input: Input) -> i64 {
    let (polymer, rules) = input;
    let mut cur_polymer: Polymer = polymer;

    println!("template: {}", cur_polymer);
    for i in 0..10 {
        cur_polymer = step(cur_polymer, rules.clone());
        println!("finished step {}", i + 1);
        // println!("polymer: {}", cur_polymer);
    }


    // let mut t: String = "abcd".to_string();
    // t.insert(2, 'x');
    // println!("test {}" ,t);


    let cnts: HashMap<char, i64> = counts(cur_polymer.clone());
    let min: i64 = *cnts.values().min().unwrap();
    let max: i64 = *cnts.values().max().unwrap();
    return max - min;
}

fn part2(input: Input) -> i64 {
    let (polymer, rules) = input;
    let mut cur_polymer: Polymer = polymer.clone();

    println!("template: {}", cur_polymer);

    let mut rule_map: HashMap<(char, char), char> = HashMap::new();
    for rule in rules {
        rule_map.insert((rule.left, rule.right), rule.insert);
    }

    // map from char to set<int>
    let mut counts: HashMap<char, i64> = HashMap::new();
    let mut pair_counts: HashMap<(char, char), i64> = HashMap::new();

    let x: Vec<char> = polymer.clone().chars().into_iter().collect::<Vec<char>>();
    let pairs: Windows<char> = x.windows(2);


    for pair in pairs {
        let l = pair[0];
        let r = pair[1];

        if !counts.contains_key(&l) {
            counts.insert(l, 0);
        }
        counts.insert(l, counts.get(&l).unwrap() + 1);

        if !pair_counts.contains_key(&(l, r)) {
            pair_counts.insert((l, r), 0);
        }
        pair_counts.insert((l,r), pair_counts.get(&(l, r)).unwrap() + 1);
    }
    // get the last one
    let last: char = polymer.chars().last().unwrap();
    if !counts.contains_key(&last) {
        counts.insert(last, 0);
    }
    counts.insert(polymer.chars().last().unwrap(), counts.get(&polymer.chars().last().unwrap()).unwrap() + 1);



    for step in 0..40 {
        for ((k1, k2), v) in pair_counts.clone().into_iter() {
            *pair_counts.get_mut(&(k1, k2)).unwrap() -= v;
            let add = rule_map.get(&(k1, k2)).unwrap();
            if !counts.contains_key(add) {
                counts.insert(*add, 0);
            }
            *counts.get_mut(add).unwrap() += v;
            if !pair_counts.contains_key(&(k1, *add)) {
                pair_counts.insert((k1, *add), 0);
            }
            *pair_counts.get_mut(&(k1, *add)).unwrap() += v;

            if !pair_counts.contains_key(&(*add, k2)) {
                pair_counts.insert((*add, k2), 0);
            }
            *pair_counts.get_mut(&(*add, k2)).unwrap() += v;
        }

        println!("finished step {}", step + 1);
    }

    let min: i64 = *counts.values().min().unwrap();
    let max: i64 = *counts.values().max().unwrap();
    return max - min;
}


// fn build_fast_polymer(old_polymer: Polymer) -> HashMap<char, HashSet<i64>> {
//     let mut res = HashMap::new();
//
//     let mut idx = 0;
//     for ch in old_polymer {
//         if !res.contains_key(ch) {
//             res.insert(ch, HashSet::new())
//         }
//         res.get(ch).unwrap().insert(idx);
//         idx += 1;
//     }
//
//     return res;
// }



// A rule like AB -> C means that when elements A and B are immediately adjacent, element C should be inserted between them
fn step(old_polymer: Polymer, rules: Vec<PairInsertionRule>) -> Polymer {
    // let x: Vec<char> = old_polymer.clone().chars().into_iter().collect::<Vec<char>>();
    // let windows: Windows<char> = x.windows(2);
    // let w: Vec<String> = windows.clone().map(|f| pair(f, rules.clone())).collect();
    // return w.concat();
    let mut new_polymer: Polymer = old_polymer.clone();
    let mut insertions: Vec<(usize, char)> = Vec::new();

    // go across, check each rule
    for i in 0..old_polymer.len()-1 {
        for rule in rules.clone() {
            if old_polymer.chars().nth(i).unwrap() == rule.left && old_polymer.chars().nth(i+1).unwrap() == rule.right {
                insertions.push(((i + 1), rule.insert));
            }
        }
    }

    // reverse insertions
    for (idx, ch) in insertions.into_iter().rev() {
        new_polymer.insert(idx, ch);
    }

    return new_polymer;
}


// map from chars to seq<position>?


fn pair(win: &[char], rules: Vec<PairInsertionRule>) -> String {
    let l = win[0];
    let r = win[1];

    let mut res = "".to_string();
    for rule in rules {
        if l == rule.left && r == rule.right {
            res.push(l);
            res.push(rule.insert);
            res.push(r);
            return res;
        }
    }
    res.push(l);
    res.push(r);
    return res;
}


fn counts(polymer: Polymer) -> HashMap<char, i64> {
    let mut cnts: HashMap<char, i64> = HashMap::new();

    for ch in polymer.clone().chars() {
        if !cnts.contains_key(&ch) {
            cnts.insert(ch, 0);
        }
        let old_v = cnts.get(&ch).unwrap();
        cnts.insert(ch, old_v + 1);
    }

    return cnts;
}

fn parse_input(lines: Vec<String>) -> Input {
    let p: Polymer = lines.clone().into_iter().nth(0).unwrap();
    let rules: Vec<PairInsertionRule> = parse_rules(lines.clone()[2..lines.clone().len()].to_vec());

    return (p, rules)
}

fn parse_rules(lines: Vec<String>) -> Vec<PairInsertionRule> {
    return lines.into_iter().map(|l| parse_rule(l)).collect();
}

fn parse_rule(line: String) -> PairInsertionRule {
    let sp: Vec<String> = line.split(" -> ").into_iter().map(|s| s.to_string()).collect();
    let left: char = sp[0].chars().nth(0).unwrap();
    let right: char = sp[0].chars().nth(1).unwrap();

    let insert: char = sp[1].chars().nth(0).unwrap();

    return PairInsertionRule { left, right, insert };
}