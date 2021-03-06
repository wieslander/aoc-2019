use std::cmp::{max, Ordering};
use std::collections::{HashMap, BinaryHeap};
use std::collections::hash_map::Entry;
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

    pub fn direction_to(&self, neighbor: &Point) -> Option<Direction> {
        for d in Direction::all() {
            if self.point_in_direction(&d) == *neighbor {
                return Some(d)
            }
        }

        None
    }
}

#[derive(Debug)]
enum Tile {
    Floor,
    Wall,
    Portal {
        name: String,
        neighbor: Point,
        destination: Point,
    },
}

type Grid = HashMap<Point, Tile>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    position: Point,
    level: i32,
    cost: i32,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        (other.level, other.cost).cmp(&(self.level, self.cost))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    grid: Grid,
    start: Point,
    goal: Point,
    center: Point,
}

impl Maze {
    pub fn from_str(s: &str) -> Maze {
        let mut raw_grid = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let x = x as i32;
                let y = y as i32;
                if ch != ' ' {
                    raw_grid.insert(Point { x, y }, ch);
                }

                max_x = max(max_x, x);
                max_y = max(max_y, y);
            }
        }

        let mut grid = HashMap::new();
        let mut portals = HashMap::new();
        let mut start = None;
        let mut goal = None;

        for (pos, ch) in &raw_grid {
            let tile = match ch {
                '#' => Some(Tile::Wall),
                '.' => Some(Tile::Floor),
                ' ' => None,
                'A'..='Z' => {
                    let neighbors = pos.neighbors();
                    let neighbors = neighbors.iter().filter(|n| raw_grid.get(&n).unwrap_or(&' ') == &'.');
                    let adjacent_floor_positions: Vec<_> = neighbors.collect();

                    if adjacent_floor_positions.len() == 0 {
                        continue;
                    }

                    let adjacent_floor_pos = adjacent_floor_positions[0];
                    let dir = adjacent_floor_pos.direction_to(pos).unwrap();

                    let adjacent_letter_pos = pos.point_in_direction(&dir);
                    let mut letter_positions = vec![*pos, adjacent_letter_pos];
                    letter_positions.sort_by_key(|p| (p.x, p.y));
                    let letters: Vec<_> = letter_positions.iter().map(|p| raw_grid[p]).collect();
                    let name = format!("{}{}", letters[0], letters[1]);

                    if name == "AA" {
                        start = Some(adjacent_floor_pos.clone());
                        None
                    } else if name == "ZZ" {
                        goal = Some(adjacent_floor_pos.clone());
                        None
                    } else {
                        portals.entry(name.clone()).or_insert(vec![]).push(pos);

                        // Placeholder destination.  Will be replaced with
                        // the real destination portal during the next pass.
                        let destination = Point { x: -1, y: -1 };
                        let portal = Tile::Portal {
                            name,
                            destination,
                            neighbor: *adjacent_floor_pos,
                        };

                        Some(portal)
                    }
                },
                _ => None,
            };

            if let Some(t) = tile {
                grid.insert(*pos, t);
            }
        }

        for positions in portals.values() {
            for (i0, i1) in &[(0, 1), (1, 0)] {
                let portal_pos = positions[*i0];
                let exit_portal_pos = positions[*i1];

                let neighbors = exit_portal_pos.neighbors();
                let exit_pos = neighbors.iter().find(|n| {
                    let tile = grid.get(&n).unwrap_or(&Tile::Wall);
                    if let Tile::Floor = tile {
                        true
                    } else {
                        false
                    }
                }).expect(&format!("Found no floor tile near {:?}", exit_portal_pos));

                if let Entry::Occupied(mut e) = grid.entry(*portal_pos) {
                    if let Tile::Portal { name: _, neighbor: _, destination } = e.get_mut() {
                        *destination = exit_pos.clone();
                    }
                }
            }
        }

        let center = Point { x: max_x / 2, y: max_y / 2 };

        Maze { grid, start: start.unwrap(), goal: goal.unwrap(), center }
    }

    fn tile(&self, position: &Point) -> &Tile {
        self.grid.get(position).unwrap_or(&Tile::Wall)
    }

    fn neighbors(&self, position: &Point, level: i32) -> Vec<(Point, i32)> {
        let mut neighbors = vec![];

        for pos in position.neighbors() {
            if (pos == self.goal || pos == self.start) && level != 0 {
                continue;
            }

            match self.tile(&pos) {
                Tile::Wall => (),
                Tile::Floor => neighbors.push((pos, level)),
                Tile::Portal { name: _, neighbor, destination } => {
                    let center_distance =
                        pos.manhattan_distance(&self.center);
                    let neighbor_center_distance =
                        neighbor.manhattan_distance(&self.center);
                    let level_diff = if center_distance < neighbor_center_distance {
                        1
                    } else {
                        -1
                    };
                    let new_level = level + level_diff;
                    if new_level >= 0 {
                        neighbors.push((*destination, new_level));
                    }
                },
            }
        }

        neighbors
    }

    fn solve(&self) -> Option<i32> {
        let mut costs = HashMap::new();
        let mut to_visit = BinaryHeap::new();

        to_visit.push(State { position: self.start, cost: 0, level: 0 });

        while let Some(State { position, cost, level }) = to_visit.pop() {
            if position == self.goal && level == 0 {
                return Some(cost);
            }

            if cost > *costs.get(&(position, level)).unwrap_or(&i32::max_value()) {
                continue;
            }

            for (neighbor, new_level) in self.neighbors(&position, level) {
                let new_cost = cost + 1;
                let min_cost = *costs.get(&(neighbor, new_level)).unwrap_or(&i32::max_value());

                if new_cost < min_cost {
                    costs.insert((neighbor, new_level), new_cost);

                    to_visit.push(State {
                        position: neighbor,
                        cost: new_cost,
                        level: new_level,
                    });
                }
            }
        }

        None
    }
}

fn main() {
    let input = get_input();
    let maze = Maze::from_str(&input);

    match maze.solve() {
        Some(steps) => println!("{}", steps),
        None => println!("No solution found"),
    }
}
