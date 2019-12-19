use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let memory: Vec<i64> = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();
    let mut p = Program::new(&memory);
    let mut count = 0;

    for x in 0..50 {
        for y in 0..50 {
            p.reset(&memory);
            p.set_input(x);
            p.set_input(y);

            count += p.pause_on_output().unwrap();
        }
    }

    println!("{}", count);
}
