use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut input: Vec<i32> = input.iter().map(|i| *i).collect();

    input[1] = 12;
    input[2] = 2;

    let mut index = 0;
    loop {
        match input[index] {
            1 => {
                let dest = input[index + 3] as usize;
                input[dest] = input[index + 1] + input[index + 2];
            }
            2 => {
                let dest = input[index + 3] as usize;
                input[dest] = input[index + 1] * input[index + 2];
            }
            99 => break,
            _ => println!(
                "something went terribly wrong (opcode {} at {})",
                input[index], index
            ),
        }

        index += 4;
    }

    input[0]
}
