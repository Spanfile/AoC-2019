use super::intcode::IntcodeVM;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let memory: Vec<i64> = input.iter().copied().collect();
    let mut vm = IntcodeVM::new(memory);
    vm.input(1);
    vm.get_next_output().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let memory: Vec<i64> = input.iter().copied().collect();
    let mut vm = IntcodeVM::new(memory);
    vm.input(5);
    vm.get_next_output().unwrap()
}
