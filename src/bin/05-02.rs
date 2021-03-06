use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let input = get_input();
    let initial_memory = input
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();
    let mut program = Program::new(&initial_memory);
    program.set_input(5);
    program.run();
    println!("{}", program.pop_output().unwrap());
}
