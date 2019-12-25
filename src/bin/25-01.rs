use std::env;
use std::io;
use std::process;
use itertools::Itertools;
use regex::Regex;
use aoc::get_input_from_file;
use aoc::intcode::Program;

struct Game {
    program: Program,
    save_states: Vec<SaveState>,
    room: String,
    room_inv: Vec<String>,
    inv: Vec<String>,

}

#[derive(Clone)]
struct SaveState {
    program: Program,
    room: String,
    inv: Vec<String>,
    room_inv: Vec<String>,
}

impl Game {
    pub fn new(memory: &Vec<i64>) -> Game {
        let program = Program::new(memory);
        let mut save_states = vec![];
        save_states.push(SaveState {
            program: program.clone(),
            inv: vec![],
            room: String::new(),
            room_inv: vec![],
        });

        Game {
            program,
            save_states,
            inv: vec![],
            room: String::new(),
            room_inv: vec![],
        }
    }

    pub fn save(&mut self) {
        self.save_states.push(SaveState {
            program: self.program.clone(),
            inv: self.inv.clone(),
            room: self.room.clone(),
            room_inv: self.room_inv.clone(),
        });
    }

    pub fn undo(&mut self) {
        self.save_states.pop().unwrap();
        let SaveState { program, inv, room, room_inv } = self.save_states.pop().unwrap();
        self.program = program;
        self.inv = inv;
        self.room = room;
        self.room_inv = room_inv;
    }

    pub fn is_running(&self) -> bool {
        self.program.is_running()
    }

    pub fn has_output(&self) -> bool {
        self.program.has_output()
    }

    pub fn needs_input(&self) -> bool {
        self.program.needs_input()
    }

    pub fn pop_output(&mut self) -> Option<char> {
        match self.program.pop_output() {
            Some(ch) => Some((ch as u8) as char),
            None => None,
        }
    }

    pub fn step(&mut self) {
        self.program.step();
    }

    pub fn bruteforce(&mut self, direction: &str) {
        let start_room = self.room.clone();
        let mut items = vec![];

        for item in &self.room_inv {
            items.push(item.clone());
        }

        for item in &self.inv {
            items.push(item.clone());
        }

        for inventory_size in (1..=items.len()).rev() {
            for inventory in items.iter().combinations(inventory_size) {
                self.ensure_inventory(&inventory);
                self.echo_command(&direction);
                self.process_and_print_output();

                if start_room != self.room {
                    return;
                }
            }
        }
    }

    fn ensure_inventory(&mut self, items: &Vec<&String>) {
        let mut to_take = vec![];
        let mut to_drop = vec![];

        for item in items {
            if !self.inv.contains(item) {
                to_take.push(item);
            }
        }

        for item in &self.inv {
            if !items.contains(&item) {
                to_drop.push(item.clone());
            }
        }

        for item in to_drop {
            self.drop(&item);
        }

        for item in to_take {
            self.take(&item);
        }
    }

    fn drop(&mut self, item: &str) {
        self.echo_command(&format!("drop {}", item));
        self.process_and_print_output();
    }

    fn take(&mut self, item: &str) {
        self.echo_command(&format!("take {}", item));
        self.process_and_print_output();
    }

    fn echo_command(&mut self, command: &str) {
        println!("{}", command);
        self.send_command(command);
    }

    fn send_command(&mut self, command: &str) {
        for ch in command.chars() {
            self.program.set_input(ch as i64);
        }

        // finish with a newline
        self.program.set_input(10);

        // consume the input
        self.step_safely();
    }

    pub fn run_input_command(&mut self) {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cmd = input.trim();
                if cmd == "undo" {
                    self.undo();
                } else if cmd.starts_with("bruteforce ") {
                    let args: Vec<_> = cmd.split(" ").collect();
                    let direction = args[1];
                    self.bruteforce(&direction);
                } else {
                    self.send_command(&cmd);
                }
            },
            Err(_) => { process::exit(0); },
        }
    }

    pub fn step_safely(&mut self) {
        while !self.has_output() && !self.needs_input() && self.is_running() {
            self.step();
        }
    }

    pub fn process_and_print_output(&mut self) {
        let mut output = String::new();

        while !self.needs_input() && self.is_running() {
            if let Some(ch) = self.pop_output() {
                output += &ch.to_string();
            }
            self.step();
        }

        self.process_output(&output);

        print!("{}", output)
    }

    pub fn process_output(&mut self, output: &str) {
        let room_re = Regex::new(r"== (?P<room_name>.*) ==").unwrap();
        let item_re = Regex::new(r"- (?P<item>.*)").unwrap();
        let take_re = Regex::new(r"You take the (?P<item>.*).").unwrap();
        let drop_re = Regex::new(r"You drop the (?P<item>.*).").unwrap();

        let mut got_room_inv = false;
        let mut got_inventory = false;
        let mut get_items = false;
        let mut items = vec![];

        for line in output.lines() {
            if get_items {
                if let Some(caps) = item_re.captures(line) {
                    let item = caps.name("item").unwrap().as_str();
                    items.push(String::from(item));
                } else {
                    get_items = false;
                }
            } else if let Some(caps) = room_re.captures(line) {
                let room = caps.name("room_name").unwrap().as_str();
                self.room = String::from(room);
                got_room_inv = true;
            } else if let Some(caps) = take_re.captures(line) {
                let item = caps.name("item").unwrap().as_str();
                self.inv.push(String::from(item));
                let room_inv_pos = self.room_inv.
                    iter()
                    .position(|i| i == &item)
                    .unwrap();
                self.room_inv.remove(room_inv_pos);
            } else if let Some(caps) = drop_re.captures(line) {
                let item = caps.name("item").unwrap().as_str();
                self.room_inv.push(String::from(item));
                let inv_pos = self.inv.
                    iter()
                    .position(|i| i == &item)
                    .unwrap();
                self.inv.remove(inv_pos);
            } else if line == "Items here:" {
                get_items = true;
                got_room_inv = true;
            } else if line == "Items in your inventory:" {
                get_items = true;
                got_inventory = true;
            } else if line == "You aren't carrying any items" {
                got_inventory = true;
            }
        }

        if got_room_inv {
            self.room_inv = items;
        } else if got_inventory {
            self.inv = items;
        }
    }

    pub fn run(&mut self) {
        let mut try_again = true;

        while try_again {
            while self.is_running() {
                if self.has_output() {
                    self.save();
                    self.process_and_print_output();
                }

                if self.needs_input() {
                    self.run_input_command();
                }

                self.step_safely();
            }

            try_again = false;
            println!("Undo?");
            let mut input = String::new();

            if let Ok(_) = io::stdin().read_line(&mut input) {
                if input.trim() == "y" {
                    self.undo();
                    try_again = true;
                    continue;
                }
            }
        }
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];
    let memory = get_input_from_file(filename)
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();
    let mut game = Game::new(&memory);
    game.run();
}
