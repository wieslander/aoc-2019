use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let input = get_input();
    let mut initial_memory: Vec<i32> = input
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();
    initial_memory[1] = 12;
    initial_memory[2] = 2;
    let mut program = Program::new(&initial_memory);
    program.run();
    println!("{}", program.read(0));
}
