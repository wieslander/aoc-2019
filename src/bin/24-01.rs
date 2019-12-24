use std::collections::{HashMap, HashSet};
use aoc::get_input;

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

type Grid = HashMap<Point, char>;

fn neighbor_bug_count(pos: &Point, grid: &Grid) -> u32 {
    let mut count = 0;

    for n in pos.neighbors() {
        let tile = *grid.get(&n).unwrap_or(&'.');
        if tile == '#' {
            count += 1;
        }
    }

    count
}

fn evolve(grid: &Grid) -> Grid {
    let mut new_grid = grid.clone();

    for y in 0..5 {
        for x in 0..5 {
            let pos = Point { x, y };
            let neighbor_bug_count = neighbor_bug_count(&pos, &grid);

            let new_tile = if grid[&pos] == '#' {
                match neighbor_bug_count {
                    1 => '#',
                    _ => '.',
                }
            } else {
                match neighbor_bug_count {
                    1..=2 => '#',
                    _ => '.',
                }
            };

            new_grid.insert(pos, new_tile);
        }
    }

    new_grid
}

fn get_biodiversity_rating(grid: &Grid) -> i64 {
    let mut rating = 0;
    let mut exponent = 0;

    for y in 0..5 {
        for x in 0..5 {
            let pos = Point { x, y };
            if grid[&pos] == '#' {
                rating += 2i64.pow(exponent);
            }
            exponent += 1;
        }
    }

    rating
}

fn print_grid(grid: &Grid) {
    for y in 0..5 {
        for x in 0..5 {
            let pos = Point { x, y };
            let ch = grid[&pos];
            print!("{}", ch);
        }
        println!();
    }
}

fn main() {
    let mut grid = HashMap::new();
    let mut ratings = HashSet::new();

    for (y, line) in get_input().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            let pos = Point { x, y };
            grid.insert(pos, ch);
        }
    }

    loop {
        print_grid(&grid);
        println!();
        let rating = get_biodiversity_rating(&grid);
        if ratings.contains(&rating) {
            println!("Iterations: {}", ratings.len());
            println!("{}", rating);
            break;
        } else {
            ratings.insert(rating);
        }

        grid = evolve(&grid);
    }
}
