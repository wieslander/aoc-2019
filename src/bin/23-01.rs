use std::process;
use aoc::intcode::Program;
use aoc::get_input;

fn main() {
    let memory = get_input()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();
    let mut programs = vec![];
    let mut queues = vec![];

    for addr in 0..50 {
        let mut p = Program::new(&memory);

        while !p.needs_input() {
            p.step();
        }
        p.set_input(addr);
        p.step();

        programs.push(p);
        queues.push(vec![]);
    }

    loop {
        for addr in 0..50 {
            let p = &mut programs[addr];
            if p.needs_input() {
                if queues[addr].is_empty() {
                    p.set_input(-1);
                } else {
                    let (x, y) = queues[addr].pop().unwrap();
                    p.set_input(x);
                    p.set_input(y);
                    p.step();
                    p.step();
                }
            }

            p.step();

            if p.has_output() {
                let dst = p.pause_on_output().unwrap() as usize;
                let x = p.pause_on_output().unwrap();
                let y = p.pause_on_output().unwrap();

                if dst == 255 {
                    println!("{}", y);
                    process::exit(0);
                } else {
                    queues[dst].insert(0, (x, y));
                }
            }
        }
    }
}
