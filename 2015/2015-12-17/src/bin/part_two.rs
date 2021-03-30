fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-17 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    Combinations::total(150, input)
}

struct Combinations {
    best: u32,
    count: u32,
}

impl Combinations {
    fn total(target: u32, containers: &str) -> u32 {
        let mut containers: Vec<u32> = containers.lines().map(|x| x.parse().unwrap()).collect();
        containers.sort_unstable();

        let mut combinations = Self {
            best: u32::MAX,
            count: 0,
        };

        combinations.generate(&containers, target, 1);
        combinations.count
    }

    fn generate(&mut self, containers: &[u32], target: u32, depth: u32) {
        for (i, x) in containers.iter().enumerate() {
            match target.checked_sub(*x) {
                Some(x) if x == 0 => self.increment(depth),
                Some(x) => self.generate(&containers[i + 1..], x, depth + 1),
                None => return,
            }
        }
    }

    fn increment(&mut self, depth: u32) {
        if depth > self.best {
            return;
        } else if depth < self.best {
            self.best = depth;
            self.count = 0;
        }

        self.count += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, Combinations::total(25, "20\n15\n10\n5\n5"));
    }
}
