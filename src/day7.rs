use super::intcode::IntcodeVM;
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
    let memory: Vec<i64> = memory.iter().copied().collect();
    let mut vm = IntcodeVM::new(memory);
    vm.input(phase);
    vm.input(signal);
    vm.get_next_output().unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let memory: Vec<i64> = input.iter().copied().collect();
    let mut max_signal = std::i64::MIN;
    let mut phases = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phases);

    for phase_perm in heap {
        let mut amp_a = IntcodeVM::new(memory.clone());
        let mut amp_b = IntcodeVM::new(memory.clone());
        let mut amp_c = IntcodeVM::new(memory.clone());
        let mut amp_d = IntcodeVM::new(memory.clone());
        let mut amp_e = IntcodeVM::new(memory.clone());

        amp_a.input(phase_perm[0]);
        amp_b.input(phase_perm[1]);
        amp_c.input(phase_perm[2]);
        amp_d.input(phase_perm[3]);
        amp_e.input(phase_perm[4]);

        let mut signal = 0;
        amp_a.input(0);

        loop {
            if let Some(amp_a_out) = amp_a.get_next_output() {
                amp_b.input(amp_a_out);
                let amp_b_out = amp_b.get_next_output().unwrap();

                amp_c.input(amp_b_out);
                let amp_c_out = amp_c.get_next_output().unwrap();

                amp_d.input(amp_c_out);
                let amp_d_out = amp_d.get_next_output().unwrap();

                amp_e.input(amp_d_out);
                signal = amp_e.get_next_output().unwrap();

                amp_a.input(signal);

            // println!(
            //     "a: {}, b: {}, c: {}, d: {}, e: {}, phase: {:?}",
            //     amp_a_out, amp_b_out, amp_c_out, amp_d_out, signal,
            // phase_perm );
            } else {
                break;
            }
        }

        if signal > max_signal {
            println!("{:?} -> {}", phase_perm, signal);
            max_signal = signal
        }
    }

    max_signal
}
