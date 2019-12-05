use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate(i32, i32);

#[derive(Debug)]
pub struct Wire(Vec<Direction>);

#[aoc_generator(day3)]
pub fn generator(input: &str) -> (Wire, Wire) {
    let mut lines: Vec<Wire> = input.split('\n').map(|s| s.parse().unwrap()).collect();
    (lines.pop().unwrap(), lines.pop().unwrap())
}

#[aoc(day3, part1)]
pub fn part1(input: &(Wire, Wire)) -> i32 {
    let (wire_a, wire_b) = input;
    let mut wire_a_path = HashSet::new();
    let mut wire_b_path = HashSet::new();

    for coord in wire_a.traverse() {
        wire_a_path.insert(coord);
    }

    for coord in wire_b.traverse() {
        wire_b_path.insert(coord);
    }

    let mut shortest = std::i32::MAX;
    for coord in wire_a_path.intersection(&wire_b_path) {
        let dist = coord.manhattan();
        if dist < shortest {
            shortest = dist
        }
    }

    shortest
}

#[aoc(day3, part2)]
pub fn part2(input: &(Wire, Wire)) -> i32 {
    let (wire_a, wire_b) = input;
    let mut wire_a_steps = HashMap::new();
    let mut wire_b_steps = HashMap::new();
    let mut wire_a_path = HashSet::new();
    let mut wire_b_path = HashSet::new();

    let wire_a_coords = wire_a.traverse();
    for (i, coord) in wire_a_coords.iter().enumerate() {
        wire_a_path.insert(coord);
        if !wire_a_steps.contains_key(coord) {
            wire_a_steps.insert(coord, i as i32 + 1);
        }
    }

    let wire_b_coords = wire_b.traverse();
    for (i, coord) in wire_b_coords.iter().enumerate() {
        wire_b_path.insert(coord);
        if !wire_b_steps.contains_key(coord) {
            wire_b_steps.insert(coord, i as i32 + 1);
        }
    }

    let mut min_steps = std::i32::MAX;
    for coord in wire_a_path.intersection(&wire_b_path) {
        let steps = wire_a_steps.get(coord).unwrap() + wire_b_steps.get(coord).unwrap();
        if steps < min_steps {
            min_steps = steps
        }
    }

    min_steps
}

impl FromStr for Direction {
    type Err = !;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        let distance = distance.parse::<i32>().unwrap();

        match direction {
            "L" => Ok(Direction::Left(distance)),
            "R" => Ok(Direction::Right(distance)),
            "U" => Ok(Direction::Up(distance)),
            "D" => Ok(Direction::Down(distance)),
            _ => panic!(),
        }
    }
}

impl FromStr for Wire {
    type Err = !;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Wire(
            s.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<Direction>>(),
        ))
    }
}

impl Wire {
    fn traverse(&self) -> Vec<Coordinate> {
        let mut origin = Coordinate(0, 0);
        let mut coords = Vec::new();

        for dir in &self.0 {
            let (delta, dist) = match dir {
                Direction::Left(dist) => (Coordinate(-1, 0), *dist),
                Direction::Right(dist) => (Coordinate(1, 0), *dist),
                Direction::Up(dist) => (Coordinate(0, -1), *dist),
                Direction::Down(dist) => (Coordinate(0, 1), *dist),
            };

            for _ in 0..dist {
                let coord = origin + delta;
                coords.push(coord);
                origin = coord;
            }
        }

        coords
    }
}

impl Coordinate {
    fn manhattan(self) -> i32 {
        (self.0 + self.1).abs()
    }
}

impl ops::Add<Coordinate> for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Coordinate) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Coordinate> for Coordinate {
    type Output = Coordinate;
    fn sub(self, rhs: Coordinate) -> Self::Output {
        Coordinate(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl ops::Mul<i32> for Coordinate {
    type Output = Coordinate;
    fn mul(self, rhs: i32) -> Self::Output {
        Coordinate(self.0 * rhs, self.1 * rhs)
    }
}
