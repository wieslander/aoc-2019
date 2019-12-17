use std::collections::HashMap;
use aoc::get_input;
use aoc::intcode::Program;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

enum Tile {
    Space,
    Scaffold,
    Droid(Direction),
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }
}

fn main() {
    let mut memory: Vec<i64> = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();

    memory[0] = 2;

    let mut p = Program::new(&memory);
    let mut grid = HashMap::new();
    let mut pos = Point { x: 0, y: 0 };

    let input = vec![
        "A,B,A,C,B,C,A,B,A,C\n",
        "R,10,L,8,R,10,R,4\n",
        "L,6,L,6,R,10\n",
        "L,6,R,12,R,12,R,10\n",
        "y\n",
    ].join("");

    for ch in input.chars() {
        p.set_input(ch as i64);
    }

    while p.is_running() {
        if let Some(ch) = p.pause_on_output() {
            if ch > 255 {
                println!("Dust collected: {}", ch);
                continue;
            }

            let out = (ch as u8) as char;
            match out {
                '\n' => {
                    pos.step(&Direction::Down);
                    pos.x = -1; // Incremented after the match block
                },
                '#' => {
                    grid.insert(pos, Tile::Scaffold);
                },
                '.' => {
                    grid.insert(pos, Tile::Space);
                },
                '<' => {
                    grid.insert(pos, Tile::Droid(Direction::Left));
                },
                '>' => {
                    grid.insert(pos, Tile::Droid(Direction::Right));
                },
                '^' => {
                    grid.insert(pos, Tile::Droid(Direction::Up));
                },
                'V' => {
                    grid.insert(pos, Tile::Droid(Direction::Down));
                },
                _ => (),
            }
            pos.x += 1;
            print!("{}", out);
        }
    }
}
