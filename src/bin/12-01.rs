use std::cell::RefCell;
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

    pub fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn potential_energy(&self) -> i32 {
        let p = &self.position;
        p.x.abs() + p.y.abs() + p.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        let v = self.velocity.borrow();
        v.x.abs() + v.y.abs() + v.z.abs()
    }
}

fn main() {
    let mut moons: Vec<Moon> = get_input().lines().map(|l| Moon::from(l)).collect();

    for _ in 0..1000 {
        for moon in &moons {
            for other in &moons {
                moon.update_velocity(&other);
            }
        }

        for moon in &mut moons {
            moon.update_position();
        }
    }

    let energy: i32 = moons.iter().map(|m| m.energy()).sum();
    println!("{}", energy);
}
