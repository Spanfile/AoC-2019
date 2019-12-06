use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<f64> {
    input.split('\n').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[f64]) -> f64 {
    input.iter().map(|n| (n / 3.0).floor() - 2.0).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[f64]) -> f64 {
    input.iter().map(|n| calculate_fuel(*n)).sum()
}

fn calculate_fuel(mass: f64) -> f64 {
    let mut fuel = (mass / 3.0).floor() - 2.0;
    if fuel <= 0.0 {
        0.0
    } else {
        if fuel > 0.0 {
            fuel += calculate_fuel(fuel);
        }
        fuel
    }
}
