use std::collections::HashMap;
use std::ops::RangeInclusive;
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
    z: i32,
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

    pub fn neighbors_2d(&self) -> Vec<Point> {
        Direction::all().iter().map(|dir| self.point_in_direction(dir)).collect()
    }

    pub fn neighbors(&self) -> Vec<Point> {
        let mut neighbors = vec![];

        for Point { x, y, z } in self.neighbors_2d() {
            let mut other_level = false;

            if x == -1 || x == 5 {
                let next_x = if x == -1 { 1 } else { 3 };
                neighbors.push(Point { x: next_x, y: 2, z: z - 1 });
                other_level = true;
            }

            if y == -1 || y == 5 {
                let next_y = if y == -1 { 1 } else { 3 };
                neighbors.push(Point { x: 2, y: next_y, z: z - 1 });
                other_level = true;
            }

            if x == 2 && y == 2 {
                if self.x == x {
                    let new_y = if y > self.y { 0 } else { 4 };
                    for x in 0..5 {
                        neighbors.push(Point { x, y: new_y, z: z + 1 });
                    }
                    other_level = true;
                } else if self.y == y {
                    let new_x = if x > self.x { 0 } else { 4 };
                    for y in 0..5 {
                        neighbors.push(Point { x: new_x, y, z: z + 1 });
                    }
                    other_level = true;
                }
            }

            if !other_level {
                neighbors.push(Point { x, y, z });
            }
        }

        neighbors
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

fn neighbor_z_range(grid: &Grid) -> RangeInclusive<i32> {
    let min_z = grid.keys().map(|Point { z, .. }| z).min().unwrap();
    let max_z = grid.keys().map(|Point { z, .. }| z).max().unwrap();

    (min_z - 1)..=(max_z + 1)
}

fn evolve(grid: &Grid) -> Grid {
    let mut new_grid = grid.clone();

    for z in neighbor_z_range(&grid) {
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }

                let pos = Point { x, y, z };
                let count = neighbor_bug_count(&pos, &grid);

                let new_tile = if *grid.get(&pos).unwrap_or(&'.') == '#' {
                    match count {
                        1 => '#',
                        _ => '.',
                    }
                } else {
                    match count {
                        1..=2 => '#',
                        _ => '.',
                    }
                };

                if new_tile == '#' || grid.contains_key(&pos) {
                    new_grid.insert(pos, new_tile);
                }
            }
        }
    }

    new_grid
}

fn bug_count(grid: &Grid) -> usize {
    grid.values().filter(|tile| *tile == &'#').count()
}

fn main() {
    let mut grid = HashMap::new();

    for (y, line) in get_input().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let x = x as i32;
                let y = y as i32;
                let pos = Point { x, y, z: 0 };
                grid.insert(pos, ch);
            }
        }
    }

    for _ in 0..200 {
        grid = evolve(&grid);
    }

    println!("{}", bug_count(&grid));
}
