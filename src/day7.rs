use super::intcode::intcode;
use aoc_runner_derive::{aoc, aoc_generator};
use permutohedron::Heap;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[i64]) -> i64 {
    let mut max_signal = std::i64::MIN;
    let mut phases = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phases);

    for phase_perm in heap {
        let amp_a = amplifier(input, phase_perm[0], 0);
        let amp_b = amplifier(input, phase_perm[1], amp_a);
        let amp_c = amplifier(input, phase_perm[2], amp_b);
        let amp_d = amplifier(input, phase_perm[3], amp_c);
        let amp_e = amplifier(input, phase_perm[4], amp_d);

        if amp_e > max_signal {
            println!("{:?} -> {}", phase_perm, amp_e);
            max_signal = amp_e
        }
    }

    max_signal
}

fn amplifier(memory: &[i64], phase: i64, signal: i64) -> i64 {
    let mut input = vec![signal, phase];
    let mut memory: Vec<i64> = memory.iter().copied().collect();
    intcode(&mut memory, &mut input)
}
