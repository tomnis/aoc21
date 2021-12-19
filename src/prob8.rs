use std::collections::HashMap;
use crate::read_lines;
use crate::util::{all_same_chars, is_superset};

#[derive(Clone, Debug)]
struct Entry { signal_patterns: Vec<String>, output_values: Vec<String> }

pub(crate) fn prob8() {
    let lines: Vec<String> = read_lines("./input/prob8.txt");
    let entries: Vec<Entry> = parse_entries(lines.clone());
    println!("parsed {} entries", entries.len());
    println!("{}", part1(entries.clone()));
    println!("{}", part2(entries.clone()));
}

// Each digit of a seven-segment display is rendered by turning on or off any of seven segments named a through g:

// you might know that only signal wires b and g are turned on, but that doesn't mean segments b and g are turned on: the only digit that uses two segments is 1, so it must mean segments c and f are meant to be on.
// With just that information, you still can't tell which wire (b/g) goes to which segment (c/f).

// In the output values, how many times do digits 1, 4, 6, or 8 appear?
// Each entry consists of ten unique signal patterns, a | delimiter, and finally the four digit output value
// aedcg db ecbdgf badfegc abfcde edb cbgfe bfdg bdgec agfbce | ecagd gcbde dbcefa bgfd



// the digits 1, 4, 7, and 8 each use a unique number of segments

// the signals which control the segments have been mixed up on each display.
// The submarine is still trying to display numbers by producing output on signal wires a through g, but those wires are connected to segments randomly
fn part1(entries: Vec<Entry>) -> i64 {
    return entries.into_iter().map(|entry| count_1_4_7_8(entry.output_values)).sum();
}

// decode the four-digit output values. What do you get if you add up all of the output values?
fn part2(entries: Vec<Entry>) -> i64 {
    return entries.into_iter().map(|e| decode_entry(e)).sum();
}

fn decode_entry(entry: Entry) -> i64 {
    //   0:      1:      2:      3:      4:
    //  aaaa    ....    aaaa    aaaa    ....
    // b    c  .    c  .    c  .    c  b    c
    // b    c  .    c  .    c  .    c  b    c
    //  ....    ....    dddd    dddd    dddd
    // e    f  .    f  e    .  .    f  .    f
    // e    f  .    f  e    .  .    f  .    f
    //  gggg    ....    gggg    gggg    ....
    //
    //   5:      6:      7:      8:      9:
    //  aaaa    aaaa    aaaa    aaaa    aaaa
    // b    .  b    .  .    c  b    c  b    c
    // b    .  b    .  .    c  b    c  b    c
    //  dddd    dddd    ....    dddd    dddd
    // .    f  e    f  .    f  e    f  .    f
    // .    f  e    f  .    f  e    f  .    f
    //  gggg    gggg    ....    gggg    gggg


    // we know 1, 7, 4, 8
    let rep_1: String = entry.signal_patterns.clone().into_iter().find(|pat| pat.to_string().len() == 2).unwrap();
    println!("{} = 1", rep_1);

    let rep_4: String = entry.signal_patterns.clone().into_iter().find(|pat| pat.to_string().len() == 4).unwrap();
    println!("{} = 4", rep_4);

    let rep_7: String = entry.signal_patterns.clone().into_iter().find(|pat| pat.to_string().len() == 3).unwrap();
    println!("{} = 7", rep_7);

    let rep_8: String = entry.signal_patterns.clone().into_iter().find(|pat| pat.to_string().len() == 7).unwrap();
    println!("{} = 8", rep_8);

    // disambiguate 2,3,5
    let rep_235: Vec<String> = entry.signal_patterns.clone().into_iter().filter(|pat| pat.to_string().len() == 5).collect();
    // 3 is a superset of 1
    let rep_3: String = rep_235.clone().into_iter().find(|rep| is_superset(rep.to_string(), rep_1.clone())).unwrap();
    println!("{} = 3", rep_3);

    // disambiguate 0,6,9
    let rep_069: Vec<String> = entry.signal_patterns.clone().into_iter().filter(|pat| pat.to_string().len() == 6).collect();
    // 6 is not a superset of 1 (but 0 and 9 are)
    let rep_6: String = rep_069.clone().into_iter().find(|pat| !is_superset(pat.to_string(), rep_1.clone())).unwrap();
    println!("{} = 6", rep_6);

    let rep_09: Vec<String> = rep_069.clone().into_iter().filter(|pat| pat.to_string() != rep_6).collect();
    // 9 is a superset of 4
    let rep_9: String = rep_09.clone().into_iter().find(|pat| is_superset(pat.to_string(), rep_4.clone())).unwrap();
    println!("{} = 9", rep_9);
    // that leaves 0
    let rep_0: String = rep_09.clone().into_iter().find(|pat| pat.to_string() != rep_9).unwrap();
    println!("{} = 0", rep_0);

    // disambiguate 2,5
    let rep_25: Vec<String> = rep_235.clone().into_iter().filter(|pat| pat.to_string() != rep_3).collect();
    // 5 is a subset of 6
    let rep_5: String = rep_25.clone().into_iter().find(|pat| is_superset(rep_6.clone(), pat.to_string())).unwrap();
    println!("{} = 5", rep_5);
    // leaves 2 leftover
    let rep_2: String = rep_25.clone().into_iter().find(|pat| pat.to_string() != rep_5.clone()).unwrap();
    println!("{} = 2", rep_2);

    // construct the direct mapping from patterns to digits
    let mut mapping: HashMap<String, char> = HashMap::new();

    mapping.insert(rep_0, '0');
    mapping.insert(rep_1, '1');
    mapping.insert(rep_2, '2');
    mapping.insert(rep_3, '3');
    mapping.insert(rep_4, '4');
    mapping.insert(rep_5, '5');
    mapping.insert(rep_6, '6');
    mapping.insert(rep_7, '7');
    mapping.insert(rep_8, '8');
    mapping.insert(rep_9, '9');

    // look up every output value in the mapping, concat, parse
    let mut r: String = "".to_string();
    for output_value in entry.output_values.clone() {
        println!("checking output value {}", output_value.clone());
        let key: String = mapping.clone().keys().find(|k| all_same_chars(output_value.clone(), k.to_string())).unwrap().to_string();
        let output_digit: char = (&*mapping.clone().get(&*key).unwrap()).clone();
        r.push(output_digit);
    }

    return r.parse().unwrap();
}

fn count_1_4_7_8(output_values: Vec<String>) -> i64 {
    return output_values.into_iter().map(|output_value| if is_1_4_7_8(output_value) {1} else{0}).sum::<i64>();
}

fn parse_entries(lines: Vec<String>) -> Vec<Entry> {
    return lines.into_iter().map(|line| parse_line(line)).collect();
}

fn parse_line(line: String) -> Entry {
    let ln: Vec<String> = line.split(" | ").map(|x| x.to_string()).into_iter().collect();
    let signal_patterns: Vec<String> = ln[0].trim().split_whitespace().map(|x| x.to_string()).collect();
    let output_values: Vec<String> = ln[1].trim().split_whitespace().map(|x| x.to_string()).collect();
    return Entry { signal_patterns: signal_patterns, output_values: output_values }
}

fn is_1_4_7_8(output_value: String) -> bool {
    let l = output_value.len();
    return l == 2 || l == 3 || l == 4 || l == 7;
}