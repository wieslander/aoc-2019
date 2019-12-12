use std::cell::RefCell;
use num::integer::lcm;
use regex::Regex;
use aoc::get_input;

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

struct Moon {
    position: Point,
    velocity: RefCell<Point>,
}

impl Moon {
    pub fn from(line: &str) -> Moon {
        let re = Regex::new(r"<x=(?P<x>-?[0-9]*), *y=(?P<y>-?[0-9]*), *z=(?P<z>-?[0-9]*)>").unwrap();
        let caps = re.captures(line).expect("Invalid position");
        let x = caps.name("x").unwrap().as_str().parse().unwrap();
        let y = caps.name("y").unwrap().as_str().parse().unwrap();
        let z = caps.name("z").unwrap().as_str().parse().unwrap();

        let position = Point { x, y, z };

        Moon {
            position,
            velocity: RefCell::new(Point { x: 0, y: 0, z: 0 }),
        }
    }

    pub fn update_velocity(&self, other: &Moon) {
        let pos = &self.position;
        let other = &other.position;
        let mut vel = self.velocity.borrow_mut();

        if pos.x < other.x {
            vel.x += 1;
        } else if pos.x > other.x {
            vel.x -= 1;
        }
        if pos.y < other.y {
            vel.y += 1;
        } else if pos.y > other.y {
            vel.y -= 1;
        }
        if pos.z < other.z {
            vel.z += 1;
        } else if pos.z > other.z {
            vel.z -= 1;
        }
    }

    pub fn update_position(&mut self) {
        let v = self.velocity.borrow();
        let p = &mut self.position;

        p.x += v.x;
        p.y += v.y;
        p.z += v.z;
    }
}

fn main() {
    let mut moons: Vec<Moon> = get_input().lines().map(|l| Moon::from(l)).collect();
    let mut steps = 0u64;
    let mut steps_x = 0u64;
    let mut steps_y = 0u64;
    let mut steps_z = 0u64;
    let initial_state_x: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.x, m.velocity.borrow().x)).collect();
    let initial_state_y: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.y, m.velocity.borrow().y)).collect();
    let initial_state_z: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.z, m.velocity.borrow().z)).collect();

    while steps_x * steps_y * steps_z == 0 {
        for moon in &moons {
            for other in &moons {
                moon.update_velocity(&other);
            }
        }

        for moon in &mut moons {
            moon.update_position();
        }

        steps += 1;

        let state_x: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.x, m.velocity.borrow().x)).collect();
        let state_y: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.y, m.velocity.borrow().y)).collect();
        let state_z: Vec<(i32, i32)> = moons.iter().map(|m| (m.position.z, m.velocity.borrow().z)).collect();

        if state_x == initial_state_x && steps_x == 0 {
            steps_x = steps;
        }

        if state_y == initial_state_y && steps_y == 0 {
            steps_y = steps;
        }

        if state_z == initial_state_z && steps_z == 0 {
            steps_z = steps;
        }
    }

    let result = lcm(lcm(steps_x, steps_y), steps_z);
    println!("{}", result);
}
