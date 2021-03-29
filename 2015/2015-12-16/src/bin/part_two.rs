use std::collections::HashMap;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-16 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    let ticker = AuntSue::from(
        "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, \
            goldfish: 5, trees: 3, cars: 2, perfumes: 1",
    );

    input
        .lines()
        .map(AuntSue::from)
        .find_map(|x| if x == ticker { Some(x.number) } else { None })
        .unwrap()
}

#[derive(Debug)]
struct AuntSue {
    number: u32,
    attributes: HashMap<String, u32>,
}

impl PartialEq for AuntSue {
    fn eq(&self, other: &Self) -> bool {
        self.attributes.iter().all(|(x, y)| {
            let ticker = &other.attributes[x];
            match x.as_str() {
                "cats" | "trees" => y > ticker,
                "pomeranians" | "goldfish" => y < ticker,
                _ => y == ticker,
            }
        })
    }
}

impl From<&str> for AuntSue {
    fn from(s: &str) -> Self {
        let sentence: Vec<&str> = s
            .split_whitespace()
            .map(|x| x.trim_end_matches(|x| x == ':' || x == ','))
            .collect();

        let number = sentence[1].parse().unwrap();
        let mut attributes = HashMap::new();

        for x in sentence.chunks(2).skip(1) {
            attributes.insert(x[0].to_string(), x[1].parse().unwrap());
        }

        Self { number, attributes }
    }
}
