use std::process;
use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let input = get_input();
    let mut initial_memory: Vec<i64> = input
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            initial_memory[1] = noun;
            initial_memory[2] = verb;
            let mut program = Program::new(&initial_memory);
            program.run();

            if program.read(0) == 19690720 {
                let res = 100 * noun + verb;
                println!("{}", res);
                process::exit(0);
            }
        }
    }

    println!("No result found");
}
