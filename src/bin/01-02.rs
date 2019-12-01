use aoc::get_input;

fn get_mass(line: &str) -> i32 {
    line.parse().expect("NaN")
}

fn fuel_req(mass: i32) -> i32 {
    let mut total_fuel_mass = 0;
    let mut fuel_mass = mass / 3 - 2;

    while fuel_mass > 0 {
        total_fuel_mass += fuel_mass;
        fuel_mass = fuel_mass / 3 - 2;
    }

    total_fuel_mass
}

fn main() {
    let input = get_input();
    let masses = input.lines().map(get_mass);
    let total_fuel_req: i32 = masses.map(fuel_req).sum();
    println!("{}", total_fuel_req);
}
