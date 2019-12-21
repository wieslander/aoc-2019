use aoc::get_input;
use aoc::intcode::Program;

fn main() {
    let memory = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();

    let mut p = Program::new(&memory);

    let input = vec![
        "NOT A J",
        "NOT B T",
        "OR T J",
        "NOT C T",
        "OR T J",
        "AND D J",
        "WALK\n"
    ].join("\n");

    for ch in input.chars() {
        p.set_input(ch as i64);
    }

    while p.is_running() {
        if let Some(ch) = p.pause_on_output() {
            if ch > 255 {
                println!("{}", ch);
                continue;
            }
            let out = (ch as u8) as char;
            print!("{}", out);
        }
    }
}
