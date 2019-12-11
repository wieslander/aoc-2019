use std::collections::HashMap;
use aoc::get_input;
use aoc::intcode::Program;

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn turn_left(&mut self) {
        let new_x = self.y;
        let new_y = -self.x;

        self.x = new_x;
        self.y = new_y;
    }

    pub fn turn_right(&mut self) {
        let new_x = -self.y;
        let new_y = self.x;

        self.x = new_x;
        self.y = new_y;
    }

    pub fn go(&mut self, direction: &Point) {
        self.x += direction.x;
        self.y += direction.y;
    }
}

fn main() {
    let input = get_input();
    let initial_memory = input
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();
    let mut program = Program::new(&initial_memory);

    let mut grid = HashMap::new();
    let mut pos = Point { x: 0, y: 0 };
    let mut direction = Point { x: 0, y: -1 };

    while program.is_running() {
        match grid.get(&(pos.x, pos.y)) {
            Some(&color) => program.set_input(color),
            None => program.set_input(0),
        }

        match program.pause_on_output() {
            Some(color) => { grid.insert((pos.x, pos.y), color); }
            None => break,
        }

        match program.pause_on_output() {
            Some(dir) => {
                match dir {
                    0 => direction.turn_left(),
                    _ => direction.turn_right(),
                }
            },
            None => break,
        }

        pos.go(&direction);
    }

    let paint_count = grid.iter().count();
    println!("{}", paint_count);
}
