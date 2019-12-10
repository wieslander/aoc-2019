use std::f64::consts::PI;
use std::process;
use std::collections::{HashSet, HashMap};
use num::integer::gcd;
use aoc::get_input;

fn get_station_location(asteroids: &HashSet::<(i32, i32)>) -> (i32, i32) {
    let mut max_count = 0;
    let mut best = (-1, -1);

    for (x, y) in asteroids {
        let mut count = 0;

        for (other_x, other_y) in asteroids {
            if (other_x, other_y) == (x, y) {
                continue;
            }

            let dx = other_x - x;
            let dy = other_y - y;
            let divisor = gcd(dx, dy);

            let step_x = match dx {
                0 => 0,
                _ => dx / divisor,
            };
            let step_y = match dy {
                0 => 0,
                _ => dy / divisor,
            };

            let mut blocking_x = x + step_x;
            let mut blocking_y = y + step_y;

            let mut blocked = false;
            while (&blocking_x, &blocking_y) != (other_x, other_y) {
                if asteroids.contains(&(blocking_x, blocking_y)) {
                    blocked = true;
                    break;
                }
                blocking_x += step_x;
                blocking_y += step_y;
            }

            if !blocked {
                count += 1;
            }

        }

        if count > max_count {
            max_count = count;
            best = (*x, *y);
        }
    }
    best
}

fn get_asteroid_directions(station: (i32, i32), asteroids: &HashSet<(i32, i32)>) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
    let mut directions = HashMap::new();

    for (x, y) in asteroids {
        if (*x, *y) == station {
            continue;
        }

        let dx = x - station.0;
        let dy = y - station.1;
        let divisor = gcd(dx, dy);

        let step_x = match dx {
            0 => 0,
            _ => dx / divisor,
        };
        let step_y = match dy {
            0 => 0,
            _ => dy / divisor,
        };
        let direction = (step_x, step_y);

        directions.entry(direction).or_insert(vec![]).push((*x, *y));
    }

    for dir_asteroids in directions.values_mut() {
        dir_asteroids.sort_by(|(x, y), (other_x, other_y)| {
            let dx = (x - station.0) as f64;
            let dy = (y - station.1) as f64;
            let other_dx = (other_x - station.0) as f64;
            let other_dy = (other_y - station.1) as f64;

            let distance = (dx.powi(2) + dy.powi(2)).sqrt();
            let other_distance = (other_dx.powi(2) + other_dy.powi(2)).sqrt();

            other_distance.partial_cmp(&distance).unwrap()
        });
    }

    directions
}

fn angle(x: i32, y: i32) -> f64 {
    let x = x as f64;
    let y = y as f64;

    let mut angle = y.atan2(x) + PI * 0.5;

    while angle < 0.0 {
        angle += 2.0 * PI;
    }

    while angle > 2.0 * PI {
        angle -= 2.0 * PI;
    }

    angle
}

fn get_sorted_directions(dirmap: &HashMap<(i32, i32), Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
    let mut directions: Vec<(i32, i32)> = dirmap.keys().map(|(x, y)| (*x, *y)).collect();

    directions.sort_by(|(x, y), (other_x, other_y)| {
        let a1 = angle(*x, *y);
        let a2 = angle(*other_x, *other_y);
        a1.partial_cmp(&a2).unwrap()
    });

    directions
}

fn main() {
    let mut asteroids = HashSet::new();
    for (y, line) in get_input().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                let pos_x = x as i32;
                let pos_y = y as i32;
                asteroids.insert((pos_x, pos_y));
            }
        }
    }

    let (station_x, station_y) = get_station_location(&asteroids);
    let mut dirmap = get_asteroid_directions((station_x, station_y), &asteroids);
    let sorted_directions = get_sorted_directions(&dirmap);

    let mut count = 0;

    loop {
        for direction in &sorted_directions {
            let targets = dirmap.get_mut(&direction).unwrap();
            if !targets.is_empty() {
                let (x, y) = targets.pop().unwrap();
                count += 1;

                if count == 200 {
                    println!("{}", x * 100 + y);
                    process::exit(0);
                }
            }
        }
    }
}
