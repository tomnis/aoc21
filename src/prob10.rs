use std::collections::VecDeque;
use crate::read_lines;

pub(crate) fn prob10() {
    let lines: Vec<String> = read_lines("./input/prob10.txt");
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));
}


// Find the first illegal character in each corrupted line of the navigation subsystem.
// What is the total syntax error score for those errors?
fn part1(lines: Vec<String>) -> i64 {
    return lines.into_iter().flat_map(|line| first_illegal_character(line)).map(|ch| score(ch)).sum();
}

// discard the corrupted lines. The remaining lines are incomplete.
// figure out the sequence of closing characters that complete all open chunks in the line
fn part2(lines: Vec<String>) -> i64 {
    let incomplete_lines: Vec<String> = lines.into_iter().filter(|line| first_illegal_character(line.to_string()).is_none()).collect();
    let closing_sequences: Vec<String> = incomplete_lines.into_iter().map(|line| closing_sequence(line)).collect();
    let mut scores: Vec<i64> = closing_sequences.into_iter().map(|seq| score_completion_sequence(seq)).collect();

    // Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and then taking the middle score
    scores.sort();
    return scores[scores.len() / 2];
}

fn closing_char(ch: char) -> char {
    if ch == '(' {
        return ')';
    }
    else if ch == '[' {
        return ']';
    }
    else if ch == '{' {
        return '}';
    }
    else if ch == '<' {
        return '>';
    }
    else {
        panic!("dont know how to close {}", ch);
    }
}

fn closing_sequence(line: String) -> String {
    let mut q: VecDeque<char> = VecDeque::new();

    for ch in line.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            q.push_back(ch);
        }
        else if q.back().map_or_else(|| false, |last_seen| matches(*last_seen,ch)) {
            q.pop_back();
        }
    }

    return q.into_iter().rev().map(|ch| closing_char(ch)).collect();
}


// Start with a total score of 0.
// Then, for each character, multiply the total score by 5 and then increase the total score by the point value
// given for the character in the following table:
//
//     ): 1 point.
//     ]: 2 points.
//     }: 3 points.
//     >: 4 points
fn score_completion_sequence(seq: String) -> i64 {
    let mut score: i64 = 0;

    for ch in seq.chars() {
        score *= 5;
        if ch == ')' {
            score += 1;
        }
        else if ch == ']' {
            score += 2;
        }
        else if ch == '}' {
            score += 3;
        }
        else if ch == '>' {
            score += 4;
        }
    }

    return score
}


fn first_illegal_character(line: String) -> Option<char> {
    let mut q: VecDeque<char> = VecDeque::new();
    for ch in line.chars() {
        if ch == '(' || ch == '[' || ch == '{' || ch == '<' {
            q.push_back(ch);
        }
        else {
            if q.back().map_or_else(|| false, |last_seen| matches(*last_seen,ch)) {
                q.pop_back();
            }
            else {
                return Some(ch);
            }
        }
    }

    return None;
}

fn matches(a: char, b: char) -> bool {
    if a == '(' {
        return b == ')';
    }
    else if a == '[' {
        return b == ']';
    }
    else if a == '{' {
        return b == '}';
    }
    else if a == '<' {
        return b == '>';
    }
    else {
        return false;
    }
}


// 477912


// ): 3 points.
// ]: 57 points.
// }: 1197 points.
// >: 25137 points
fn score(ch: char) -> i64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unknown char {}", ch.to_string())
    }
}