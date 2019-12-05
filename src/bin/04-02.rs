use aoc::get_input;

fn is_increasing(pwd: &Vec<char>) -> bool {
    for i in 1..pwd.len() {
        if pwd[i] < pwd[i - 1] {
            return false;
        }
    }
    true
}

fn has_double_digits(pwd: &Vec<char>) -> bool {
    let mut current_digit = pwd[0];
    let mut current_group_length = 0;

    for i in 0..pwd.len() {
        if pwd[i] != current_digit {
            if current_group_length == 2 {
                /*
                let pwd_vec: Vec<String> = pwd.iter().map(|c| c.to_string()).collect();
                println!("{:?} - {} len {} after pos {}", pwd_vec.join(""), current_digit, current_group_length, i);
                */
                return true;
            }
            current_digit = pwd[i];
            current_group_length = 0;
        }
        current_group_length += 1;
    }

    current_group_length == 2
}

fn is_valid_password(pwd: &u32) -> bool {
    let pwd = pwd.to_string().chars().collect();
    is_increasing(&pwd) && has_double_digits(&pwd)
}

fn main() {
    let input = get_input();
    let ranges: Vec<u32> = input.split('-').map(|x| x.parse().unwrap()).collect();

    if let [start, end] = ranges[0..2] {
        let password_count = (start..=end).filter(is_valid_password).count();
        println!("{}", password_count);
    }
}
