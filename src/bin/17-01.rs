use std::collections::HashMap;
use aoc::get_input;
use aoc::intcode::Program;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }
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

    pub fn point_in_direction(&self, direction: &Direction) -> Point {
        let mut p = self.clone();
        p.step(direction);
        p
    }

    pub fn neighbors(&self) -> Vec<Point> {
        Direction::all().iter().map(|dir| self.point_in_direction(dir)).collect()
    }
}

fn is_scaffold(point: &Point, grid: &HashMap<Point, Tile>) -> bool {
    if let Some(tile) = grid.get(point) {
        if let Tile::Scaffold = tile {
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn get_intersection_sum(grid: &HashMap<Point, Tile>) -> i32 {
    let mut sum = 0;

    for (pos, tile) in grid {
        if let Tile::Scaffold = tile {
            if pos.neighbors().iter().all(|&n| is_scaffold(&n, grid)) {
                sum += pos.x * pos.y
            }
        }
    }

    sum
}

fn main() {
    let memory = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();

    let mut p = Program::new(&memory);
    let mut grid = HashMap::new();
    let mut pos = Point { x: 0, y: 0 };

    while p.is_running() {
        if let Some(ch) = p.pause_on_output() {
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
        }
    }

    println!("{}", get_intersection_sum(&grid));
}
