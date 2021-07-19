fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-15 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let starts: Vec<usize> = input.split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let first = Generator { previous: starts[0], factor: 16807 };
    let other = Generator { previous: starts[1], factor: 48271 };

    first.zip(other).take(40000000)
        .filter(|x| x.0 & 0xFFFF == x.1 & 0xFFFF)
        .count()
}

struct Generator {
    previous: usize,
    factor: usize,
}

impl Iterator for Generator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.previous = self.previous * self.factor % 2147483647;
        Some(self.previous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(588, solve_puzzle("65 8921"));
    }
}
