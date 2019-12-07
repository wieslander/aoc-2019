use std::collections::HashMap;
use aoc::get_input;

fn main() {
    let input = get_input();
    let orbits: Vec<Vec<&str>> = input
        .lines()
        .map(|l|  l.split(')').collect())
        .collect();

    let mut orbit_map = HashMap::new();

    for o in orbits {
        let parent = o[0];
        let child = o[1];

        let parent_neighbors = orbit_map.entry(parent).or_insert(vec![]);
        parent_neighbors.push(child);

        let child_neighbors = orbit_map.entry(child).or_insert(vec![]);
        child_neighbors.push(parent);
    }

    let santa_distance = get_distance(&orbit_map, "YOU", "SAN");

    println!("{}", santa_distance - 2);
}

fn get_distance(
    orbit_map: &HashMap<&str, Vec<&str>>,
    from: &str,
    to: &str,
    ) -> i32
{
    let mut distance_map = HashMap::new();
    let mut remaining = vec![(from, 0)];

    while remaining.len() > 0 && !distance_map.contains_key(to) {
        let (node, distance) = remaining.pop().unwrap();
        distance_map.insert(node, distance);

        if let Some(neighbors) = orbit_map.get(node) {
            for neighbor in neighbors {
                if !distance_map.contains_key(neighbor) {
                    remaining.push((&neighbor, distance + 1));
                }
            }
        }
    }

    *distance_map.get(to).expect("Didn't find destination")
}
