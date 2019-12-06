use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn generator(input: &str) -> (i32, i32) {
    let args: Vec<i32> = input.split('-').map(|s| s.parse().unwrap()).collect();
    (args[0], args[1])
}

#[aoc(day4, part1)]
pub fn part1(input: &(i32, i32)) -> i32 {
    let (from, to) = *input;
    let mut total = 0;

    for i in from..to {
        if criteria1(i) && criteria2(i) {
            total += 1;
        }
    }

    total
}

#[aoc(day4, part2)]
pub fn part2(input: &(i32, i32)) -> i32 {
    let (from, to) = *input;
    let mut total = 0;

    for i in from..to {
        if criteria1(i) && criteria2(i) && criteria3(i) {
            total += 1;
        }
    }

    total
}

fn criteria1(i: i32) -> bool {
    let i_str = i.to_string();

    for window in i_str.chars().collect::<Vec<char>>().windows(2) {
        if window[0] == window[1] {
            return true;
        }
    }

    false
}

fn criteria2(i: i32) -> bool {
    let i_str = i.to_string();

    for window in i_str.chars().collect::<Vec<char>>().windows(2) {
        if window[0] > window[1] {
            return false;
        }
    }

    true
}

fn criteria3(i: i32) -> bool {
    let i_str = i.to_string();
    let mut counts = HashMap::new();

    for c in i_str.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts.values().any(|v| *v == 2)
}
