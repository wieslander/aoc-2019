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

fn visualize_grid(grid: &HashMap<(i64, i64), i64>) -> String {
    let mut top_left = Point { x: 0, y: 0 };
    let mut bottom_right = Point { x: 0, y: 0 };

    for &(x, y) in grid.keys() {
        if x < top_left.x {
            top_left.x = x;
        }
        if y < top_left.y {
            top_left.y = y;
        }
        if x > bottom_right.x {
            bottom_right.x = x;
        }
        if y > bottom_right.y {
            bottom_right.y = y;
        }
    }

    let height = bottom_right.y - top_left.y + 1;
    let width = bottom_right.x - top_left.x + 1;
    let row_offset = top_left.y;
    let col_offset = top_left.x;

    let mut output = vec![];

    for row in 0..height {
        let line: Vec<&str> = (0..width).map(|col| {
            let x = col + col_offset;
            let y = row + row_offset;
            match grid.get(&(x, y)) {
                Some(1) => "#",
                Some(0) => " ",
                None => " ",
                _ => "_",
            }
        }).collect();
        output.push(line.join(""));
    }

    output.join("\n")
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

    grid.insert((pos.x, pos.y), 1);

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

    println!("{}", visualize_grid(&grid));
}
