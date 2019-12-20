use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use aoc::get_input;

#[derive(Clone, Debug)]
struct Edge {
    position: Point,
    key: char,
    cost: u32,
}

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
    positions: Vec<Point>,
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

#[derive(Clone, Eq, PartialEq)]
struct EdgeState {
    position: Point,
    cost: u32,
}

impl Ord for EdgeState {
    fn cmp(&self, other: &EdgeState) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for EdgeState {
    fn partial_cmp(&self, other: &EdgeState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Vault {
    grid: HashMap<Point, Tile>,
    players: Vec<Point>,
}

impl Vault {
    pub fn from_str(input: &str) -> Vault {
        let mut grid = HashMap::new();
        let mut center = Point {
            x: usize::max_value(),
            y: usize::max_value(),
        };

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = Point { x, y };
                let mut tile = Tile::from(&ch);

                if ch == '@' {
                    center.x = x;
                    center.y = y;
                    tile = Tile::Wall;
                }

                grid.insert(pos, tile);
            }
        }

        for pos in center.neighbors() {
            grid.insert(pos, Tile::Wall);
        }

        let mut players = vec![];

        for x_offset in &[-1, 1] {
            for y_offset in &[-1, 1] {
                let x = ((center.x as i32) + x_offset) as usize;
                let y = ((center.y as i32) + y_offset) as usize;
                players.push(Point { x, y });
            }
        }

        Vault { grid, players }
    }

    fn edges(&self, start: &Point, keys: &Vec<char>) -> Vec<Edge> {
        let mut costs = HashMap::new();
        let mut edges = vec![];
        let mut to_visit = BinaryHeap::new();

        to_visit.push(EdgeState { position: *start, cost: 0 });

        while let Some(EdgeState { position, cost }) = to_visit.pop() {
            if let Tile::Key(key) = self.tile(&position) {
                if !keys.contains(&key) {
                    edges.push(Edge { position, cost, key });
                    continue;
                }
            }

            let previous_cost = *costs.get(&(position, keys.clone()))
                .unwrap_or(&u32::max_value());

            if cost > previous_cost {
                continue;
            }

            let neighbor_cost = cost + 1;

            for neighbor in position.neighbors() {
                let next = match self.tile(&neighbor) {
                    Tile::Wall => None,
                    Tile::Floor | Tile::Key(_) => Some(EdgeState {
                        position: neighbor,
                        cost: neighbor_cost,
                    }),
                    Tile::Door(id) => {
                        if keys.contains(&id) {
                            Some(EdgeState {
                                position: neighbor,
                                cost: neighbor_cost,
                            })
                        } else {
                            None
                        }
                    },
                };

                if let Some(EdgeState { position, cost }) = next {
                    let costs_key = (position, keys.clone());
                    let previous_cost = *costs.get(&costs_key)
                        .unwrap_or(&u32::max_value());

                    if cost < previous_cost {
                        costs.insert(costs_key, cost);
                        to_visit.push(EdgeState { position, cost });
                    }
                }
            }
        }

        edges
    }

    fn solve(&mut self) -> Option<u32> {
        let mut to_visit = BinaryHeap::new();
        let mut costs = HashMap::new();
        let total_key_count = self.key_count();

        to_visit.push(State {
            positions: self.players.clone(),
            cost: 0,
            keys: vec![],
        });

        while let Some(State { positions, cost, keys }) = to_visit.pop() {
            if keys.len() == total_key_count {
                return Some(cost);
            }

            let costs_key = (positions.clone(), keys.clone());
            let previous_cost = *costs.get(&costs_key)
                .unwrap_or(&u32::max_value());

            if cost > previous_cost {
                continue;
            }

            for (i, pos) in positions.iter().enumerate() {
                for Edge { position, key, cost: edge_cost } in self.edges(pos, &keys) {
                    let mut new_positions = positions.clone();
                    new_positions[i] = position;

                    let mut keys = keys.clone();
                    keys.push(key);
                    keys.sort();

                    let costs_key = (positions.clone(), keys.clone());
                    let cost = cost + edge_cost;
                    let previous_cost = *costs.get(&costs_key)
                        .unwrap_or(&u32::max_value());

                    if cost < previous_cost {
                        costs.insert(costs_key, cost);
                        to_visit.push(State {
                            positions: new_positions.clone(),
                            keys: keys.clone(),
                            cost,
                        });
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
