use std::collections::{HashMap, HashSet};
use aoc::get_input;

struct Segment {
    direction: char,
    distance: u32,
}

impl Segment {
    fn new(s: &str) -> Segment {
        let chars: Vec<char> = s.chars().collect();
        let direction = chars[0];
        let distance: u32 = (&s[1..]).parse().expect("NaN");
        Segment { direction, distance }
    }
}

fn get_step_counts(path: &Vec<Segment>) -> HashMap<(i32, i32), u32> {
    let mut squares = HashMap::new();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut steps = 0;

    for segment in path {
        for _ in 0..segment.distance {
            match segment.direction {
                'U' => y -= 1,
                'D' => y += 1,
                'R' => x += 1,
                'L' => x -= 1,
                _ => panic!("Unknown direction"),
            }
            steps += 1;
            squares.entry((x, y)).or_insert(steps);
        }
    }

    squares
}

fn main() {
    let paths: Vec<Vec<Segment>> = get_input()
        .lines()
        .map(|l| l.split(',').map(Segment::new).collect())
        .collect();

    let step_counts: Vec<HashMap<(i32, i32), u32>> =
        paths.iter().map(get_step_counts).collect();
    let square_sets: Vec<HashSet<&(i32, i32)>> =
        step_counts.iter().map(|s| s.keys().collect()).collect();
    let intersections = square_sets[0].intersection(&square_sets[1]);
    let mut distances: Vec<u32> = intersections.map(|&pos| {
        step_counts[0][pos] + step_counts[1][pos]
    }).collect();
    distances.sort();

    println!("{}", distances[0]);
}
