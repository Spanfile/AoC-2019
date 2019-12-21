use aoc_runner_derive::{aoc, aoc_generator};
use std::io::{Cursor, Read};

#[derive(Debug)]
pub struct Image {
    width: usize,
    height: usize,
    layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct Layer {
    digits: Vec<u8>,
}

impl Layer {
    pub fn new<T: Read>(source: &mut T, width: usize, height: usize) -> Self {
        let mut digits = vec![0; width * height];
        source.read_exact(&mut digits).unwrap();
        Layer {
            digits: digits.into_iter().map(|d| d - 48).collect(),
        }
    }
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Image {
    let width = 25;
    let height = 6;

    let mut layers = Vec::new();
    let mut cursor = Cursor::new(input);

    while cursor.position() < input.len() as u64 {
        layers.push(Layer::new(&mut cursor, width, height));
    }

    Image {
        width,
        height,
        layers,
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &Image) -> usize {
    let mut fewest_zeros = std::usize::MAX;
    let mut answer = 0;

    for layer in &input.layers {
        let zeros = layer.digits.iter().filter(|d| **d == 0).count();

        if zeros < fewest_zeros {
            fewest_zeros = zeros;

            let ones = layer.digits.iter().filter(|d| **d == 1).count();
            let twos = layer.digits.iter().filter(|d| **d == 2).count();
            answer = ones * twos;
        }
    }

    answer
}

#[aoc(day8, part2)]
pub fn part2(input: &Image) -> String {
    let mut chars = vec!['\n'];

    for h in 0..input.height {
        for w in 0..input.width {
            let i = (h * input.width) + w;

            for layer in &input.layers {
                if layer.digits[i] != 2 {
                    chars.push(match layer.digits[i] {
                        0 => ' ',
                        1 => '#',
                        _ => panic!(),
                    });
                    break;
                }
            }
        }

        chars.push('\n');
    }

    chars.iter().collect()
}
