use std::cmp::{min, Eq, PartialEq};
use std::clone::Clone;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Copy;
use std::thread;
use std::time::Duration;
use pancurses;
use aoc::get_input;
use aoc::intcode::Program;

const DROID: u32 = 1;
const WALL: u32 = 2;
const FLOOR: u32 = 3;
const OXYGEN_SYSTEM: u32 = 4;
const UNKNOWN: u32 = 5;
const OXYGEN: u32 = 6;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }

    pub fn step(&mut self, direction: i64) {
        match direction {
            1 => self.y -= 1,
            2 => self.y += 1,
            3 => self.x -= 1,
            4 => self.x += 1,
            _ => panic!("Unknown direction: {}", direction),
        }
    }

    pub fn point_in_direction(&self, direction: i64) -> Point {
        let mut p = self.clone();
        p.step(direction);
        p
    }

    pub fn neighbors(&self) -> Vec<Point> {
        (1..=4).map(|dir| self.point_in_direction(dir)).collect()
    }

    pub fn direction(&self, other: &Point) -> i64 {
        for dir in 1..=4 {
            if self.point_in_direction(dir) == *other {
                return dir;
            }
        }

        panic!("{:?} is not a neighbor of {:?}", other, self);
    }
}

enum Tile {
    Droid,
    Wall,
    Floor,
    OxygenSystem,
    Oxygen,
    Unknown,
}

struct RoutePlanner<'a> {
    grid: &'a HashMap<Point, Tile>,
    to_visit: HashSet<Point>,
    parents: HashMap<Point, Point>,
    g_score: HashMap<Point, u32>,
    f_score: HashMap<Point, u32>,
}

impl<'a> RoutePlanner<'a> {
    pub fn new(grid: &'a HashMap<Point, Tile>) -> RoutePlanner<'a> {
        RoutePlanner {
            grid,
            to_visit: HashSet::new(),
            parents: HashMap::new(),
            g_score: HashMap::new(),
            f_score: HashMap::new(),
        }
    }

    fn get_next_node(&self) -> Option<Point> {
        if !self.to_visit.is_empty() {
            let mut nodes: Vec<&Point> = self.to_visit.iter().collect();
            nodes.sort_by_key(|p| self.f_score[p]);
            let current = nodes[0];
            Some(current.clone())
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<Point> {
        if let Some(next) = self.get_next_node() {
            self.to_visit.remove(&next);
            Some(next)
        } else {
            None
        }
    }

    fn g(&self, node: &Point) -> u32 {
        match self.g_score.get(node) {
            Some(&score) => score,
            None => u32::max_value()
        }
    }

    fn visit(&mut self, current: &Point, neighbor: &Point, goal: &Point) {
        let tentative_g_score = self.g(&current) + 1;
        if tentative_g_score < self.g(&neighbor) {
            self.parents.insert(*neighbor, *current);
            self.g_score.insert(*neighbor, tentative_g_score);
            let h = neighbor.manhattan_distance(&goal);
            self.f_score.insert(*neighbor, self.g(neighbor) + h);
            self.to_visit.insert(*neighbor);
        }
    }

    pub fn find_optimal_route(&mut self, start: &Point, goal: &Point) -> Vec<Point> {
        self.to_visit.insert(*start);
        self.g_score.insert(*start, 0);
        self.f_score.insert(*start, 0);

        while let Some(current) = self.pop() {
            if current == *goal {
                return reconstruct_path(&current, &self.parents);
            }

            for neighbor in current.neighbors() {
                if neighbor == *goal {
                    self.visit(&current, &neighbor, goal);
                } else if let Some(tile) = self.grid.get(&neighbor) {
                    if let Tile::Floor | Tile::OxygenSystem = tile {
                        self.visit(&current, &neighbor, goal);
                    }
                }
            }
        }

        panic!("Could not find route to {:?}", goal);
    }
}

fn render(window: &pancurses::Window, grid: &HashMap<Point, Tile>, droid: &Point) {
    let mut min_x = 0;
    let mut min_y = 0;

    for p in grid.keys() {
        min_x = min(p.x, min_x);
        min_y = min(p.y, min_y);
    }

    let offset_x = if min_x < 0 { -min_x } else { 0 };
    let offset_y = if min_y < 0 { -min_y } else { 0 };

    for (pos, tile) in grid {
        let (mut output, mut color_pair) = match tile {
            Tile::Droid => (" ● ", DROID),
            Tile::Unknown => ("░░░", UNKNOWN),
            Tile::Wall => ("   ", WALL),
            Tile::Floor => (" · ", FLOOR),
            Tile::OxygenSystem => (" █ ", OXYGEN_SYSTEM),
            Tile::Oxygen => ("   ", OXYGEN),
        };

        if pos == droid && color_pair != OXYGEN {
            output = " ● ";
            color_pair = DROID;
        }

        let attr = pancurses::COLOR_PAIR(color_pair);
        window.attron(attr);
        let x = (pos.x + offset_x) * 3;
        let y = pos.y + offset_y;
        window.mvprintw(y, x, output);
        window.attroff(attr);
    }

    window.refresh();
}

fn examine_square(pos: &Point, tile: Tile, grid: &mut HashMap<Point, Tile>) {
    grid.insert(*pos, tile);

    for x in -1..=1 {
        let p = Point { x: pos.x + x, y: pos.y };
        grid.entry(p).or_insert(Tile::Unknown);
    }

    for y in -1..=1 {
        let p = Point { x: pos.x, y: pos.y + y };
        grid.entry(p).or_insert(Tile::Unknown);
    }
}

fn oxygen_system_location(grid: &HashMap<Point, Tile>) -> Option<Point> {
    for (pos, tile) in grid {
        match tile {
            Tile::OxygenSystem => return Some(*pos),
            _ => continue,
        }
    }

    None
}

fn find_nearest_unknown(grid: &HashMap<Point, Tile>, droid: &Point) -> Option<Point> {
    let mut unknowns = vec![];

    for (pos, tile) in grid {
        if let Tile::Unknown = tile {
            unknowns.push(pos);
        }
    }

    unknowns.sort_by_key(|pos| droid.manhattan_distance(pos));

    if unknowns.len() > 0 {
        Some(*unknowns[0])
    } else {
        None
    }
}

fn get_next_move(droid: &Point, goal: &Point, grid: &HashMap<Point, Tile>) -> i64 {
    let route = find_optimal_route(droid, goal, grid);
    droid.direction(&route[1])
}

fn reconstruct_path(node: &Point, parents: &HashMap<Point, Point>) -> Vec<Point> {
    let mut current = node;
    let mut path = vec![*current];

    while parents.contains_key(current) {
        current = parents.get(current).unwrap();
        path.insert(0, *current);
    }

    path
}

fn find_optimal_route(start: &Point, goal: &Point, grid: &HashMap<Point, Tile>) -> Vec<Point> {
    let mut planner = RoutePlanner::new(grid);
    planner.find_optimal_route(start, goal)
}

fn plot_route(window: &pancurses::Window, route: &Vec<Point>, droid: &Point, grid: &mut HashMap<Point, Tile>) {
    let end = route.len() - 1;
    for point in route.iter().take(end) {
        grid.insert(*point, Tile::Droid);
        render(&window, &grid, &droid);
        sleep();
    }
}

fn is_filled(grid: &HashMap<Point, Tile>) -> bool {
    for tile in grid.values() {
        if let Tile::Wall | Tile::Oxygen = tile {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn tiles_to_fill(grid: &HashMap<Point, Tile>) -> Vec<Point> {
    let mut tiles = vec![];

    for (pos, tile) in grid {
        if let Tile::Oxygen = tile {
            for neighbor in pos.neighbors() {
                if let Tile::Floor | Tile::Droid = grid.get(&neighbor).unwrap() {
                    tiles.push(neighbor);
                }
            }
        }
    }

    tiles
}

fn fill_step(grid: &mut HashMap<Point, Tile>) {
    for pos in tiles_to_fill(grid) {
        grid.insert(pos, Tile::Oxygen);
    }
}

fn fill(window: &pancurses::Window, grid: &mut HashMap<Point, Tile>, start: &Point, droid: &Point) -> u32 {
    let mut count = 0;

    grid.insert(*start, Tile::Oxygen);
    render(window, grid, droid);

    while !is_filled(grid) {
        sleep();
        fill_step(grid);
        render(window, grid, droid);
        count += 1;
    }

    count
}

fn sleep() {
    thread::sleep(Duration::from_millis(25));
}

fn main() {
    let input = get_input();
    let initial_memory = input
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();
    let mut program = Program::new(&initial_memory);
    let mut grid = HashMap::new();

    let window = pancurses::initscr();
    pancurses::start_color();
    pancurses::curs_set(0);

    pancurses::init_pair(
        DROID as i16,
        pancurses::COLOR_RED,
        pancurses::COLOR_BLACK);
    pancurses::init_pair(
        WALL as i16,
        pancurses::COLOR_YELLOW,
        pancurses::COLOR_WHITE);
    pancurses::init_pair(
        FLOOR as i16,
        pancurses::COLOR_CYAN,
        pancurses::COLOR_BLACK);
    pancurses::init_pair(
        OXYGEN_SYSTEM as i16,
        pancurses::COLOR_BLUE,
        pancurses::COLOR_BLACK);
    pancurses::init_pair(
        OXYGEN as i16,
        pancurses::COLOR_BLACK,
        pancurses::COLOR_BLUE);
    pancurses::init_pair(
        UNKNOWN as i16,
        pancurses::COLOR_WHITE,
        pancurses::COLOR_BLACK);

    let start = Point { x: 30, y: 25 };
    let mut droid = start.clone();
    examine_square(&droid, Tile::Floor, &mut grid);
    render(&window, &grid, &droid);
    thread::sleep(Duration::from_millis(2000));

    while let Some(unknown) = find_nearest_unknown(&grid, &droid) {
        let direction = get_next_move(&droid, &unknown, &grid);
        program.set_input(direction);
        let immediate_target = droid.point_in_direction(direction);

        match program.pause_on_output() {
            Some(0) => {
                grid.insert(immediate_target, Tile::Wall);
            },
            Some(1) => {
                droid.step(direction);
                examine_square(&droid, Tile::Floor, &mut grid);
            },
            Some(2) => {
                droid.step(direction);
                examine_square(&droid, Tile::OxygenSystem, &mut grid);
            },
            _ => (),
        }

        render(&window, &grid, &droid);
        sleep();
    }

    let goal = oxygen_system_location(&grid).unwrap();
    let route = find_optimal_route(&start, &goal, &grid);

    plot_route(&window, &route, &droid, &mut grid);
    thread::sleep(Duration::from_millis(500));

    let fill_steps = fill(&window, &mut grid, &goal, &droid);
    thread::sleep(Duration::from_millis(2000));
    pancurses::endwin();

    println!("{}", fill_steps);
}
