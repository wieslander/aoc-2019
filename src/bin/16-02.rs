use std::collections::HashMap;
use aoc::get_input;

fn main() {
    let input = get_input();
    let digits: Vec<i64> = input.trim().chars().map(|c| c.to_string().parse().unwrap()).collect();
    let offset: usize = input[0..7].parse().unwrap();
    let mut cache = HashMap::new();
    let max_index = digits.len() * 10000;

    let digit = |k| {
        let index = k % digits.len();
        digits[index]
    };

    for i in 0..=100 {
        for n in (offset..max_index).rev() {
            let key = (n, i);
            if i == 0 || n == max_index - 1 {
                cache.insert(key, digit(n));
            } else {
                let res = (cache[&(n, i - 1)] + cache[&(n + 1, i)]).abs() % 10;
                cache.insert(key, res);
            }
        }
    }

    let result: Vec<String> = (offset..(offset + 8)).map(|n| cache[&(n, 100)]).map(|d| d.to_string()).collect();

    println!("{}", result.join(""));
}
