use aoc::get_input;

fn is_increasing(pwd: &Vec<char>) -> bool {
    for i in 1..pwd.len() {
        if pwd[i] < pwd[i - 1] {
            return false;
        }
    }
    true
}

fn has_repeating_digits(pwd: &Vec<char>) -> bool {
    for i in 1..pwd.len() {
        if pwd[i] == pwd[i - 1] {
            return true;
        }
    }
    false
}

fn is_valid_password(pwd: &u32) -> bool {
    let pwd = pwd.to_string().chars().collect();
    is_increasing(&pwd) && has_repeating_digits(&pwd)
}

fn main() {
    let input = get_input();
    let ranges: Vec<u32> = input.split('-').map(|x| x.parse().unwrap()).collect();

    if let [start, end] = ranges[0..2] {
        let password_count = (start..=end).filter(is_valid_password).count();
        println!("{}", password_count);
    }
}
