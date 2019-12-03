use std::collections::HashSet;
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

fn visited_squares(path: &Vec<Segment>) -> HashSet<(i32, i32)> {
    let mut squares = HashSet::new();
    let mut x = 0i32;
    let mut y = 0i32;

    for segment in path {
        for _ in 0..segment.distance {
            match segment.direction {
                'U' => y -= 1,
                'D' => y += 1,
                'R' => x += 1,
                'L' => x -= 1,
                _ => panic!("Unknown direction"),
            }
            squares.insert((x, y));
        }
    }

    squares
}

fn main() {
    let paths: Vec<Vec<Segment>> = get_input()
        .lines()
        .map(|l| l.split(',').map(Segment::new).collect())
        .collect();

    let square_sets: Vec<HashSet<(i32, i32)>> =
        paths.iter().map(visited_squares).collect();
    let intersections = square_sets[0].intersection(&square_sets[1]);
    let mut distances: Vec<i32> = intersections.map(|(x, y)| {
        x.abs() + y.abs()
    }).collect();

    distances.sort();

    println!("{}", distances[0]);
}
