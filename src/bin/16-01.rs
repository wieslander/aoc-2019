use aoc::get_input;

struct PatternIterator {
    repeat_count: usize,
    position: usize,
}

impl PatternIterator {
    pub fn new(repeat_count: usize) -> PatternIterator {
        PatternIterator { repeat_count, position: 0 }
    }
}

impl Iterator for PatternIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let pattern = [0, 1, 0, -1];
        self.position += 1;
        let index = (self.position / self.repeat_count) % 4;
        Some(pattern[index])
    }
}

fn fft(digits: &Vec<i64>) -> Vec<i64> {
    (1..=digits.len()).map(|pattern_repeat_count| fft_step(digits, pattern_repeat_count)).collect()
}

fn fft_step(digits: &Vec<i64>, pattern_repeat_count: usize) -> i64 {
    let multipliers = PatternIterator::new(pattern_repeat_count);
    let total: i64 = digits
        .iter()
        .zip(multipliers)
        .map(|(d, m)| d * m)
        .sum();
    (total % 10).abs()
}

fn main() {
    let input = get_input();
    let mut digits: Vec<i64> = input.trim().chars().map(|c| c.to_string().parse().unwrap()).collect();

    for _ in 0..100 {
        digits = fft(&digits);
    }

    let result_vec: Vec<String> = digits.iter().take(8).map(|d| d.to_string()).collect();

    println!("{}", result_vec.join(""));
}
