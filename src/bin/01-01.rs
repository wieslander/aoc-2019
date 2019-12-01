use aoc::get_input;

fn get_mass(line: &str) -> u32 {
    line.parse().expect("NaN")
}

fn fuel_req(mass: u32) -> u32 {
    mass / 3 - 2
}

fn main() {
    let input = get_input();
    let masses = input.lines().map(get_mass);
    let total_fuel_req: u32 = masses.map(fuel_req).sum();
    println!("{}", total_fuel_req);
}
