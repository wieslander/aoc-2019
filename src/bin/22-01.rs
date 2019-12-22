use regex::Regex;
use aoc::get_input;

enum Shuffle {
    NewStack,
    Cut(i32),
    Deal(i32),
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
    let shuffles = input
        .lines()
        .map(|l| Shuffle::from(l));

    let size = 10007;
    let mut card = 2019;

    for shuffle in shuffles {
        match shuffle {
            Shuffle::NewStack => {
                card = (size - 1) - card;
            },
            Shuffle::Cut(pos) => {
                card = (size + card - pos) % size;
            },
            Shuffle::Deal(increment) => {
                card = (card * increment) % size;
            }
        }
    }

    println!("{}", card);
}
