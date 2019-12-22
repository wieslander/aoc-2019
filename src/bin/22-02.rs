use regex::Regex;
use aoc::get_input;

enum Shuffle {
    NewStack,
    Cut(i64),
    Deal(i64),
}

impl Shuffle {
    pub fn from(line: &str) -> Shuffle {
        let new_stack = Regex::new(r"^deal into new stack$").unwrap();
        let cut = Regex::new(r"^cut (?P<count>-?[0-9]*)$").unwrap();
        let deal = Regex::new(r"^deal with increment (?P<increment>[0-9]*)").unwrap();

        if new_stack.is_match(line) {
            return Shuffle::NewStack;
        } else if cut.is_match(line) {
            let caps = cut.captures(line).unwrap();
            let count = caps.name("count").unwrap().as_str().parse().unwrap();
            return Shuffle::Cut(count);
        } else if deal.is_match(line) {
            let caps = deal.captures(line).unwrap();
            let inc = caps.name("increment").unwrap().as_str().parse().unwrap();
            return Shuffle::Deal(inc);
        }

        panic!("Could not parse line {}", line);
    }
}

fn main() {
    let input = get_input();
    let shuffles: Vec<_> = input
        .lines()
        .map(|l| Shuffle::from(l))
        .collect();

    /*
    let size = 119315717514047;
    let iterations = 101741582076661i64;
    let mut card = 2020;
    */
    let size = 10007;
    let iterations = 1;
    let mut card = 6326;

    for shuffle in shuffles.iter().rev() {
        match shuffle {
            Shuffle::NewStack => {
                card = (size - 1) - card;
            },
            Shuffle::Cut(pos) => {
                card = (size + card + pos) % size;
            },
            Shuffle::Deal(increment) => {
                // card2 = (card * increment) % size;
                let mut new_card = 0;
                while (new_card * increment - card) % size != 0 {
                    println!("{}", (new_card * increment - card) % size);
                    new_card += 1;
                }
                card = new_card;
                /*
                let m = card / increment;
                let k = card % increment;
                card = (k * (increment + 1) + m) % size;
                */
            }
        }
    }

    /*
    for i in 0..iterations {

        if i % 10000 == 0 {
            let progress = ((i + 1) as f64) / (iterations as f64);
            println!("Progress: {} %", progress * 100.0);
        }
    }
    */

    println!("{}", card);
}
