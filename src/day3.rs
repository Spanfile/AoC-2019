use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use std::ops;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Coordinate(i32, i32);

#[derive(Debug, Copy, Clone)]
struct Segment {
    from: Coordinate,
    to: Coordinate,
}

#[derive(Debug)]
pub struct Wire(Vec<Segment>);

#[aoc_generator(day3)]
pub fn generator(input: &str) -> (Wire, Wire) {
    let mut lines: Vec<Wire> = input.split('\n').map(|s| s.parse().unwrap()).collect();
    (lines.pop().unwrap(), lines.pop().unwrap())
}

#[aoc(day3, part1)]
pub fn part1(input: &(Wire, Wire)) -> i32 {
    let (wire_a, wire_b) = input;
    let mut shortest_dist = std::i32::MAX;

    for (i, seg1) in wire_a.0.iter().enumerate() {
        for seg2 in wire_b.0.iter().skip(i) {
            if let Some(intersection) = seg1.intersection(*seg2) {
                let dist = intersection.manhattan();
                if dist != 0 && dist < shortest_dist {
                    shortest_dist = dist;
                }
            }
        }
    }

    shortest_dist
}

// #[aoc(day3, part2)]
// pub fn part2(input: &[i32]) -> i32 {
//     0
// }

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
        let mut origin = Coordinate(0, 0);
        let mut segments = Vec::new();

        for dir in s
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<Direction>>()
        {
            let to = match dir {
                Direction::Left(dist) => Coordinate(origin.0 - dist, origin.1),
                Direction::Right(dist) => Coordinate(origin.0 + dist, origin.1),
                Direction::Up(dist) => Coordinate(origin.0, origin.1 - dist),
                Direction::Down(dist) => Coordinate(origin.0, origin.1 + dist),
            };

            segments.push(Segment { from: origin, to });
            origin = to;
        }

        Ok(Wire(segments))
    }
}

impl Segment {
    fn intersection(&self, other: Segment) -> Option<Coordinate> {
        let p = self.from;
        let q = other.from;
        let r = self.to - p;
        let s = other.to - q;

        let u = (q - p).cross(r) / r.cross(s);

        if r.cross(s) != 0.0 && u >= 0.0 && u <= 1.0 {
            Some(p + p * u as i32)
        } else {
            None
        }
    }
}

impl Coordinate {
    fn cross(&self, other: Coordinate) -> f32 {
        (self.0 * other.0 - self.1 * other.1) as f32
    }

    fn manhattan(&self) -> i32 {
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
