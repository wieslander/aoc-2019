use std::env;
use std::io;
use std::process;
use aoc::get_input_from_file;
use aoc::intcode::Program;

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];
    let memory = get_input_from_file(filename)
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();
    let mut p = Program::new(&memory);
    let mut save_states = vec![];
    save_states.push(p.clone());
    let mut try_again = true;

    while try_again {
        while p.is_running() {
            if p.has_output() {
                save_states.push(p.clone());

                while !p.needs_input() && p.is_running() {
                    if let Some(ch) = p.pop_output() {
                        let out = (ch as u8) as char;
                        print!("{}", out);
                    }
                    p.step();
                }
            }

            if p.needs_input() {
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_) => {
                        if input.trim() == "undo" {
                            save_states.pop().unwrap();
                            p = save_states.pop().unwrap();
                            continue;
                        } else {
                            for ch in input.chars() {
                                p.set_input(ch as i64);
                            }
                        }
                    },
                    Err(_) => { process::exit(0); },
                }
            }

            while !p.has_output() && !p.needs_input() && p.is_running() {
                p.step();
            }
        }

        try_again = false;
        println!("Undo?");
        let mut input = String::new();

        if let Ok(_) = io::stdin().read_line(&mut input) {
            if input.trim() == "y" {
                save_states.pop().unwrap();
                p = save_states.pop().unwrap();
                try_again = true;
                continue;
            }
        }
    }
}
