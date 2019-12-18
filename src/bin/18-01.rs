use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use aoc::get_input;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
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
enum Tile {
    Wall,
    Floor,
    Key(char),
    Door(char),
}

impl Tile {
    fn from(ch: &char) -> Tile {
        match ch {
            '#' => Tile::Wall,
            '.' | '@' => Tile::Floor,
            _ => {
                if ch.is_ascii_lowercase() {
                    Tile::Key(*ch)
                } else {
                    Tile::Door(ch.to_ascii_lowercase())
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
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

#[derive(Clone, Eq, PartialEq)]
struct State {
    position: Point,
    cost: u32,
    keys: Vec<char>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Vault {
    grid: HashMap<Point, Tile>,
    player: Point,
}

impl Vault {
    pub fn from_str(input: &str) -> Vault {
        let mut grid = HashMap::new();
        let mut player = Point {
            x: usize::max_value(),
            y: usize::max_value(),
        };

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = Point { x, y };
                let tile = Tile::from(&ch);
                grid.insert(pos, tile);

                if ch == '@' {
                    player.x = x;
                    player.y = y;
                }
            }
        }

        Vault { grid, player }
    }

    fn solve(&mut self) -> Option<u32> {
        let mut to_visit = BinaryHeap::new();
        let mut costs = HashMap::new();
        let total_key_count = self.key_count();

        to_visit.push(State {
            position: self.player,
            cost: 0,
            keys: vec![],
        });

        while let Some(State { position, cost, keys }) = to_visit.pop() {
            if keys.len() == total_key_count {
                return Some(cost);
            }

            let previous_cost = *costs.get(&(position, keys.clone()))
                .unwrap_or(&u32::max_value());

            if cost > previous_cost {
                continue;
            }

            for neighbor in position.neighbors() {
                let next = match self.tile(&neighbor) {
                    Tile::Wall => None,
                    Tile::Floor => {
                        Some(State {
                            position: neighbor,
                            cost: cost + 1,
                            keys: keys.clone(),
                        })
                    },
                    Tile::Key(id) => {
                        let mut keys = keys.clone();
                        if !keys.contains(&id) {
                            keys.push(id);
                            keys.sort();
                        }
                        Some(State {
                            position: neighbor,
                            cost: cost + 1,
                            keys,
                        })
                    },
                    Tile::Door(id) => {
                        if keys.contains(&id) {
                            Some(State {
                                position: neighbor,
                                cost: cost + 1,
                                keys: keys.clone(),
                            })
                        } else {
                            None
                        }
                    },
                };

                if let Some(State { position, keys, cost }) = next {
                    let previous_cost = *costs.get(&(position, keys.clone()))
                        .unwrap_or(&u32::max_value());

                    if cost < previous_cost {
                        costs.insert((position, keys.clone()), cost);
                        to_visit.push(State { position, keys, cost });
                    }
                }
            }
        }

        None
    }

    fn key_count(&self) -> usize {
        let keys: Vec<&Tile> = self.grid.values().filter(|tile| {
            if let Tile::Key(_) = tile {
                true
            } else {
                false
            }
        }).collect();
        keys.len()
    }

    fn tile(&self, pos: &Point) -> Tile {
        *self.grid.get(pos).unwrap()
    }
}

fn main() {
    let input = get_input();
    let mut vault = Vault::from_str(&input);
    match vault.solve() {
        Some(moves) => println!("{}", moves),
        None => println!("No solution found"),
    }
}
