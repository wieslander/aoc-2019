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
    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

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

    pub fn direction(&self, other: &Point) -> Direction {
        for dir in Direction::all() {
            if self.point_in_direction(&dir) == *other {
                return dir;
            }
        }

        panic!("{:?} is not a neighbor of {:?}", other, self);
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
    let mut memory: Vec<i64> = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();

    memory[0] = 2;

    let mut p = Program::new(&memory);
    let mut grid = HashMap::new();
    let mut pos = Point { x: 0, y: 0 };

    let input_lines = vec![
        "A,B\n",
        "R,10,L,8,R,10\n",
        "R,4,L,6,L,6,R,10\n",
        "R\n",
        "y\n",
    ];
    let input = input_lines.join("");

    for ch in input.chars() {
        println!("{}", ch as i64);
        p.set_input(ch as i64);
    }

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
            print!("{}", out);
        }
    }

    println!("{}", get_intersection_sum(&grid));
}
