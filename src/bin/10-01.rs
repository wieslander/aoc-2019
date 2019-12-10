use std::cmp;
use std::collections::HashSet;
use num::integer::gcd;
use aoc::get_input;

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

    // let mut sight_counts = HashMap::new();

    let mut max_count = 0;

    for (x, y) in &asteroids {
        let mut count = 0;

        for (other_x, other_y) in &asteroids {
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

        max_count = cmp::max(count, max_count);
    }

    println!("{}", max_count);
}
