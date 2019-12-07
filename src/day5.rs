use super::intcode::intcode;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut memory: Vec<i64> = input.iter().copied().collect();
    let mut input = vec![1];
    intcode(&mut memory, &mut input)
}

#[aoc(day5, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let mut memory: Vec<i64> = input.iter().copied().collect();
    let mut input = vec![5];
    intcode(&mut memory, &mut input)
}
