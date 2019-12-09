use itertools::Itertools;

use std::cmp;
use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let initial_memory = get_input()
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();

    let mut max_output = 0;
    let mut amps = vec![];

    for _ in 0..5 {
        let amp = Program::new(&initial_memory);
        amps.push(amp);
    }

    for phases in (5..10).permutations(5) {
        let mut propagated_value = 0;

        for (amp, &phase) in amps.iter_mut().zip(&phases) {
            amp.reset(&initial_memory);
            amp.set_input(phase);
        }

        while amps[amps.len() - 1].is_running() {
            for amp in amps.iter_mut() {
                amp.set_input(propagated_value);

                if let Some(output) = amp.pause_on_output() {
                    propagated_value = output;
                }
            }
        }

        max_output = cmp::max(propagated_value, max_output);
    }

    println!("{}", max_output);
}
