use std::collections::HashMap;
use aoc::get_input;

struct ReactionPart<'a> {
    chemical: &'a str,
    qty: u64,
}

impl<'a> ReactionPart<'a> {
    pub fn new(s: &'a str) -> ReactionPart<'a> {
        let tokens: Vec<&str> = s.split(" ").collect();
        let qty = tokens[0].parse().unwrap();
        let chemical = tokens[1];

        ReactionPart { chemical, qty }
    }
}

struct Reaction<'a> {
    output: ReactionPart<'a>,
    inputs: Vec<ReactionPart<'a>>,
}

struct Solver<'a> {
    conversions: HashMap<&'a str, Reaction<'a>>,
    supply: HashMap<&'a str, u64>,
    consumed: HashMap<&'a str, u64>,
}

impl<'a> Solver<'a> {
    pub fn new(input: &'a str) -> Solver<'a> {
        let conversions = get_conversion_map(input);
        Solver {
            conversions,
            supply: HashMap::new(),
            consumed: HashMap::new(),
        }
    }

    pub fn satisfy(&mut self, chemical: &'a str, qty: u64) {
        let current_qty = self.qty(chemical);
        let mut output_qty = self.get_output_qty(chemical);

        if output_qty == 0 {
            output_qty = qty - current_qty;
        }

        let mut reaction_count = 0;
        let diff: i64 = qty as i64 - current_qty as i64;

        if diff > 0 {
            reaction_count = diff / output_qty as i64;
            if diff % output_qty as i64 != 0 {
                reaction_count += 1;
            }
        }

        if reaction_count > 0 {
            for input in self.inputs(chemical, reaction_count as u64) {
                self.satisfy(input.chemical, input.qty);
                self.consume(input.chemical, input.qty);
            }
            self.increase(chemical, output_qty * reaction_count as u64);
        }
    }

    fn get_output_qty(&self, chemical: &str) -> u64 {
        match self.conversions.get(chemical) {
            Some(reaction) => reaction.output.qty,
            None => 0,
        }
    }

    fn inputs(&self, chemical: &'a str, reaction_count: u64) -> Vec<ReactionPart<'a>> {
        match self.conversions.get(chemical) {
            Some(reaction) => reaction
                .inputs
                .iter()
                .map(|part| ReactionPart {
                    chemical: part.chemical,
                    qty: part.qty * reaction_count,
                }).collect(),
            None => vec![],
        }
    }

    fn consume(&mut self, chemical: &'a str, qty: u64) {
        self.supply.entry(chemical).and_modify(|e| *e -= qty);
        self.consumed.entry(chemical)
            .and_modify(|e| *e += qty)
            .or_insert(qty);
    }

    fn increase(&mut self, chemical: &'a str, qty: u64) {
        self.supply.entry(chemical)
            .and_modify(|e| *e += qty)
            .or_insert(qty);
    }

    fn qty(&self, chemical: &str) -> u64 {
        match self.supply.get(chemical) {
            Some(qty) => *qty,
            None => 0,
        }
    }
}

fn get_conversion_map(input: &str) -> HashMap<&str, Reaction> {
    let mut map = HashMap::new();

    for line in input.trim().lines() {
        let sides: Vec<&str> = line.split(" => ").collect();
        let output = ReactionPart::new(&sides[1]);
        let inputs = sides[0].split(", ").map(|p| ReactionPart::new(&p)).collect();
        map.insert(output.chemical, Reaction { inputs, output });
    }

    map
}

fn main() {
    let input = get_input();
    let mut solver = Solver::new(&input);
    solver.satisfy("FUEL", 1);
    println!("{}", solver.consumed.get("ORE").unwrap());
}
