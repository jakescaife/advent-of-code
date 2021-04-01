use std::collections::HashSet;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-19 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let replacements: Vec<_> = input
        .lines()
        .filter_map(|x| {
            let mut split = x.split(" => ");
            Some((split.next()?, split.next()?))
        })
        .collect();

    let molecule = input.lines().last().unwrap();
    let mut distinct = HashSet::new();

    for (pattern, replacement) in replacements {
        for (index, _) in molecule.match_indices(pattern) {
            let mut new_molecule = molecule.to_string();
            new_molecule.replace_range(index..index + pattern.len(), replacement);
            distinct.insert(new_molecule);
        }
    }

    distinct.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        assert_eq!(4, solve_puzzle("H => HO\nH => OH\nO => HH\nHOH"));
        assert_eq!(7, solve_puzzle("H => HO\nH => OH\nO => HH\nHOHOHO"));
    }
}
