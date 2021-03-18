use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-09 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    let mut journey = Journey::from(input);
    let mut minimum_distance = journey.total_distance();

    while journey.next_permutation().is_some() {
        minimum_distance = minimum_distance.min(journey.total_distance());
    }

    minimum_distance
}

#[derive(PartialEq, Eq, Hash)]
struct Path {
    start: String,
    finish: String,
}

struct Journey {
    cities: Vec<String>,
    costs: HashMap<Path, u32>,
}

impl Path {
    fn new(start: &str, finish: &str) -> Self {
        let mut start = start.to_string();
        let mut finish = finish.to_string();

        if start.cmp(&finish) == Ordering::Greater {
            std::mem::swap(&mut start, &mut finish);
        }

        Self { start, finish }
    }
}

impl Journey {
    fn total_distance(&self) -> u32 {
        self.cities
            .iter()
            .zip(self.cities.iter().skip(1))
            .map(|x| Path::new(x.0, x.1))
            .map(|x| self.costs[&x])
            .sum()
    }

    fn next_permutation(&mut self) -> Option<()> {
        let pivot = self
            .cities
            .windows(2)
            .rev()
            .position(|x| x[0] < x[1])
            .map(|x| self.cities.len() - x - 2)?;

        let successor = self
            .cities
            .iter()
            .rev()
            .position(|x| x > &self.cities[pivot])
            .map(|x| self.cities.len() - 1 - x)?;

        self.cities.swap(pivot, successor);
        self.cities[pivot + 1..].reverse();
        Some(())
    }
}

impl From<&str> for Journey {
    fn from(s: &str) -> Self {
        let mut cities = HashSet::new();
        let mut costs = HashMap::new();

        for line in s.lines() {
            let line = line.replace("to ", "");
            let line = line.replace("= ", "");
            let line: Vec<_> = line.split_whitespace().collect();

            let start = line[0];
            let finish = line[1];
            let distance: u32 = line[2].parse().unwrap();

            cities.insert(start.to_string());
            cities.insert(finish.to_string());
            costs.insert(Path::new(start, finish), distance);
        }

        let mut cities: Vec<_> = cities.into_iter().collect();
        cities.sort();

        Self { cities, costs }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        let data = "London to Dublin = 464\n\
                    London to Belfast = 518\n\
                    Dublin to Belfast = 141";

        assert_eq!(605, solve_puzzle(data));
    }
}
