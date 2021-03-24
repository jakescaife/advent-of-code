use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-13 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i32 {
    let mut seating = Seating::from(input);
    let mut optimal_happiness = seating.total_happiness();

    while seating.next_permutation().is_some() {
        optimal_happiness = optimal_happiness.max(seating.total_happiness());
    }

    optimal_happiness
}

#[derive(PartialEq, Eq, Hash)]
struct Pair {
    person1: String,
    person2: String,
}

impl Pair {
    fn new(person1: &str, person2: &str) -> Self {
        let person1 = person1.to_string();
        let person2 = person2.to_string();
        Self { person1, person2 }
    }
}

struct Seating {
    seats: Vec<String>,
    costs: HashMap<Pair, i32>,
}

impl Seating {
    fn total_happiness(&self) -> i32 {
        self.seats
            .iter()
            .zip(self.seats.iter().cycle().skip(1))
            .map(|x| (Pair::new(x.0, x.1), Pair::new(x.1, x.0)))
            .map(|x| self.costs[&x.0] + self.costs[&x.1])
            .sum()
    }

    fn next_permutation(&mut self) -> Option<()> {
        let pivot = self
            .seats
            .windows(2)
            .rev()
            .position(|x| x[0] < x[1])
            .map(|x| self.seats.len() - x - 2)?;

        let successor = self
            .seats
            .iter()
            .rev()
            .position(|x| x > &self.seats[pivot])
            .map(|x| self.seats.len() - 1 - x)?;

        self.seats.swap(pivot, successor);
        self.seats[pivot + 1..].reverse();
        Some(())
    }
}

impl From<&str> for Seating {
    fn from(s: &str) -> Self {
        let mut seats = HashSet::new();
        let mut costs = HashMap::new();

        for line in s.lines() {
            let sentence: Vec<&str> = line.split_whitespace().collect();
            let name = sentence[0];
            let next = sentence[10].trim_end_matches('.');

            let mut happiness: i32 = sentence[3].parse().unwrap();
            happiness *= match sentence[2] {
                "gain" => 1,
                _ => -1,
            };

            seats.insert(name.to_string());
            costs.insert(Pair::new(name, next), happiness);
        }

        let mut seats: Vec<String> = seats.into_iter().collect();
        seats.sort();

        Self { seats, costs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "Alice would gain 54 happiness units by sitting next to Bob.\n\
                    Alice would lose 79 happiness units by sitting next to Carol.\n\
                    Alice would lose 2 happiness units by sitting next to David.\n\
                    Bob would gain 83 happiness units by sitting next to Alice.\n\
                    Bob would lose 7 happiness units by sitting next to Carol.\n\
                    Bob would lose 63 happiness units by sitting next to David.\n\
                    Carol would lose 62 happiness units by sitting next to Alice.\n\
                    Carol would gain 60 happiness units by sitting next to Bob.\n\
                    Carol would gain 55 happiness units by sitting next to David.\n\
                    David would gain 46 happiness units by sitting next to Alice.\n\
                    David would lose 7 happiness units by sitting next to Bob.\n\
                    David would gain 41 happiness units by sitting next to Carol.";

        assert_eq!(330, solve_puzzle(data));
    }
}
