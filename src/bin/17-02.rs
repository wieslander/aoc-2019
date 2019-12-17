use std::collections::HashMap;
use aoc::get_input;
use aoc::intcode::Program;

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
}

fn read_grid(program: &mut Program) -> HashMap<Point, Tile> {
    let mut grid = HashMap::new();
    let mut end_of_grid = false;
    let mut pos = Point { x: 0, y: 0 };

    while !end_of_grid {
        let ch = (program.pause_on_output().unwrap() as u8) as char;
        match ch {
            '\n' => {
                if pos.x == 0 {
                    // Double newline means we've received the full grid
                    end_of_grid = true;
                } else {
                    pos.step(&Direction::Down);
                    pos.x = -1; // Incremented after the match block
                }
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

    grid
}

fn get_droid(grid: &HashMap<Point, Tile>) -> (Point, Direction) {
    for (pos, tile) in grid {
        if let Tile::Droid(direction) = tile {
            return (*pos, *direction);
        }
    }

    panic!("Could not find droid");
}

fn tile(pos: &Point, grid: &HashMap<Point, Tile>) -> Tile {
    *grid.get(pos).unwrap_or(&Tile::Space)
}

fn get_simple_path(grid: &HashMap<Point, Tile>) -> String {
    let (mut droid, mut dir) = get_droid(&grid);
    let mut path = vec![];
    let mut line_length = 0;

    loop {
        let next_pos = droid.point_in_direction(&dir);
        let next_tile = tile(&next_pos, grid);

        if let Tile::Scaffold = next_tile {
            line_length += 1;
            droid.step(&dir);
        } else {
            for new_dir in Direction::all() {
                let next_pos = droid.point_in_direction(&new_dir);
                if let Tile::Scaffold = tile(next_pos) {
                }
            }
        }
    }

    path.join("")
}

fn main() {
    let mut memory: Vec<i64> = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();

    memory[0] = 2;

    /*
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
    */

    let mut p = Program::new(&memory);
    let mut grid = read_grid(&mut p);
    let mut simple_path = get_simple_path(&grid);

    println!("{}", simple_path);
}
