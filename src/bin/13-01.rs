use std::collections::HashMap;
use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let input = get_input();
    let initial_memory = input
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();
    let mut program = Program::new(&initial_memory);
    let mut grid = HashMap::new();

    while program.is_running() {
        if let Some(x) = program.pause_on_output() {
            let y = program.pause_on_output().unwrap();
            let tile = program.pause_on_output().unwrap();

            grid.insert((x, y), tile);
        }
    }

    let block_count = grid.values().filter(|t| **t == 2).count();
    println!("{}", block_count);
}
