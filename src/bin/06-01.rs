use std::collections::HashMap;
use aoc::get_input;

/*
struct OrbitTree {
    object: String,
    orbit_depth: u32,
    children: Vec<OrbitTree>,
}

impl OrbitTree {
    fn build(orbits: Vec<(&str, &str)>) -> OrbitTree {
        let mut orbit_map = HashMap::new();

        for (parent, child) in orbits {
            let children = orbit_map.entry(parent).or_insert(vec![]);
            children.push(child);
        }

        let mut trees = HashMap::new();
        let mut remaining_objects = vec![("COM", None, 0)];

        while remaining_objects.len() > 0 {
            let (object, parent_name, orbit_depth)
                = remaining_objects.pop().unwrap();
            let node = OrbitTree::new(object, orbit_depth);
            trees.insert(object, node);

            match parent_name {
                Some(name) => {
                    let mut parent = trees.get(name).unwrap();
                    parent.add_child(node);
                },
                None => (),
            }

            if let Some(children) = orbit_map.get(object) {
                for child in children {
                    remaining_objects.push((&child, Some(&object), orbit_depth + 1));
                }
            }
        }

        *trees.get("COM").unwrap()
    }

    fn new(object: &str, orbit_depth: u32) -> OrbitTree {
        OrbitTree {
            object: String::from(object),
            orbit_depth,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: OrbitTree) {
        self.children.push(child);
    }

    fn total_orbit_count(&self) -> u32 {
        self.orbit_depth
    }
}
*/

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
        let children = orbit_map.entry(parent).or_insert(vec![]);
        children.push(child);
    }

    let mut total_orbit_count = 0;
    let mut remaining_counts = vec![("COM", 0)];

    while remaining_counts.len() > 0 {
        let (object_name, orbit_count) = remaining_counts.pop().unwrap();
        total_orbit_count += orbit_count;

        if let Some(children) = orbit_map.get(object_name) {
            for child in children {
                remaining_counts.push((&child, orbit_count + 1));
            }
        }
    }

    println!("{}", total_orbit_count);
}
