use itertools::Itertools;

use std::cmp;
use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let initial_memory: Vec<i32> = get_input()
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();

    let mut max_output = 0;
    let mut amp = Program::new(&initial_memory);

    for phases in (0..5).permutations(5) {
        let mut propagated_value = 0;

        for &phase in &phases {
            amp.reset(&initial_memory);
            amp.set_input(phase);
            amp.set_input(propagated_value);
            amp.run();
            propagated_value = amp.pop_output().unwrap();
        }

        max_output = cmp::max(propagated_value, max_output);
    }

    println!("{}", max_output);
}
