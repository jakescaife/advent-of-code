use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-06 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let mut blocks: Vec<usize> = input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    next_repeat(&mut blocks);
    next_repeat(&mut blocks)
}

fn next_repeat(blocks: &mut Vec<usize>) -> usize {
    let mut visited = HashSet::new();
    let bank_size = blocks.len();

    while visited.insert(blocks.clone()) {
        let count = blocks.iter().fold(0, |s, x| s.max(*x));
        let index = blocks.iter().position(|&x| x == count).unwrap();
        blocks[index] = 0;

        for x in 1..=count {
            blocks[(index + x) % bank_size] += 1;
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(4, solve_puzzle("0 2 7 0"));
    }
}
