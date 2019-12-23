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
    let mut idle_cycles = vec![];
    let mut nat = None;
    let mut last_sent_y = None;

    for addr in 0..50 {
        let mut p = Program::new(&memory);

        while !p.needs_input() {
            p.step();
        }
        p.set_input(addr);
        p.step();

        programs.push(p);
        queues.push(vec![]);
        idle_cycles.push(0);
    }

    loop {
        for addr in 0..50 {
            let p = &mut programs[addr];

            while !p.needs_input() && !p.has_output() {
                p.step();
            }

            let mut is_idle = true;

            if p.needs_input() {
                if queues[addr].is_empty() {
                    p.set_input(-1);
                } else {
                    let (x, y) = queues[addr].pop().unwrap();
                    p.set_input(x);
                    p.set_input(y);
                    p.step();
                    p.step();
                    is_idle = false;
                }
            } else {
                is_idle = false;
            }

            p.step();

            if p.has_output() {
                is_idle = false;

                let dst = p.pause_on_output().unwrap() as usize;
                let x = p.pause_on_output().unwrap();
                let y = p.pause_on_output().unwrap();

                if dst == 255 {
                    // println!("NAT: ({}, {})", x, y);
                    nat = Some((x, y));
                } else {
                    // println!("{} -> {}: ({}, {})", addr, dst, x, y);
                    queues[dst].insert(0, (x, y));
                }
            }

            if is_idle {
                idle_cycles[addr] += 1;
            } else {
                idle_cycles[addr] = 0;
            }
        }

        if idle_cycles.iter().all(|count| *count > 100) {
            if let Some((x, y)) = nat {
                println!("Restarting with {:?}", (x, y));
                queues[0].push((x, y));

                if let Some(prev_y) = last_sent_y {
                    if y == prev_y {
                        println!("{}", prev_y);
                        process::exit(0);
                    }
                }

                last_sent_y = Some(y);
            }
        }
    }
}
