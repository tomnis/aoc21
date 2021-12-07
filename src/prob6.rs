use std::collections::HashMap;
use crate::read_lines;

pub(crate) fn prob6() {
    let lines: Vec<String> = read_lines("./input/prob6.txt");
    // println!("{}", part1(lines.clone()));
    println!("{}", part2(lines.clone()));
}

// Surely, each lanternfish creates a new lanternfish once every 7 days
// model each fish as a single number that represents the number of days until it creates a new lanternfish.
// How many lanternfish would there be after 80 days
fn part1(lines: Vec<String>) -> i64 {
    let state: String = lines.clone()[0].clone();
    // let test_input: String = "3,4,3,1,2".to_string();
    return simulate_fish(state, 80);
}

fn part2(lines: Vec<String>) -> i64 {
    let state: String = lines.clone()[0].clone();
    let test_input: String = "3,4,3,1,2".to_string();
    return simulate_fish_fast(state, 256);
}

fn simulate_fish(state: String, days: i64) -> i64 {
    let mut lanternfish: Vec<i64> = state.split(",").map(|x| x.parse().unwrap()).collect();

    for day in 0..days {
        let mut new_fish: Vec<i64> = Vec::new();

        // loop over fish
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                new_fish.push(8);
                lanternfish[i] = 6;
            }
            else {
                lanternfish[i] -= 1;
            }
        }

        lanternfish.append(&mut new_fish.clone());
        println!("day {} finished, total fish: {} ", day, lanternfish.len());
    }
    return lanternfish.len() as i64;
}


fn simulate_fish_fast(state: String, days: i64) -> i64 {
    let mut lanternfish: Vec<i64> = state.split(",").map(|x| x.parse().unwrap()).collect();

    // count of fish in each "slot"
    let mut fish_reproduce_slots: HashMap<i64, i64> = HashMap::new();
    for slot in 0..10 {
        fish_reproduce_slots.insert(slot, 0);
    }

    for i in lanternfish {
        let old = fish_reproduce_slots.get(&i).unwrap();
        let new = old + 1;
        fish_reproduce_slots.insert(i, new);
    }

    // 115247851307 too low
    for day in 0..days {
        // fish with 0 days remaining create new fish in the "8" slot
        // The new lanternfish starts with an internal timer of 8 and does not start counting down until the next day.
        let num_new_fish: i64 = *fish_reproduce_slots.get(&(0 as i64)).unwrap();

        for i in 0..8 {
            fish_reproduce_slots.insert(i, *fish_reproduce_slots.get(&(i + 1 as i64)).unwrap());
        }
        // need to reset slot 6


        fish_reproduce_slots.insert(8, num_new_fish);
        let old_6 = *fish_reproduce_slots.get(&(6 as i64)).unwrap();
        fish_reproduce_slots.insert(6, old_6 + num_new_fish);
    }

    let mut sum: i64  = 0;
    for val in fish_reproduce_slots.values().into_iter() {
        sum += *val;
    }
    return sum;
}

// A lanternfish that creates a new fish resets its timer to 7.
