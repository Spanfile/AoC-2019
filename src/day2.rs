use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut input: Vec<i32> = input.iter().copied().collect();

    input[1] = 12;
    input[2] = 2;

    intcode(input)
}

#[aoc(day2, part2)]
pub fn part2(input: &[i32]) -> i32 {
    let input: Vec<i32> = input.iter().copied().collect();

    for verb in 0..99 {
        for noun in 0..99 {
            let mut memory = input.clone();
            memory[1] = noun;
            memory[2] = verb;

            if intcode(memory) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}

fn intcode(mut memory: Vec<i32>) -> i32 {
    let mut index = 0;
    loop {
        match memory[index] {
            1 => {
                let i1 = memory[index + 1] as usize;
                let i2 = memory[index + 2] as usize;
                let dest = memory[index + 3] as usize;
                memory[dest] = memory[i1] + memory[i2];
            }
            2 => {
                let i1 = memory[index + 1] as usize;
                let i2 = memory[index + 2] as usize;
                let dest = memory[index + 3] as usize;
                memory[dest] = memory[i1] * memory[i2];
            }
            99 => break,
            _ => println!(
                "something went terribly wrong (opcode {} at {})",
                memory[index], index
            ),
        }

        index += 4;
    }

    memory[0]
}
