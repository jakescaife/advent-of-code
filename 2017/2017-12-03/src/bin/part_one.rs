fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-03 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let target = input.trim().parse().unwrap();
    let mut pattern = Pattern::new();

    (0..).map(|_| pattern.next_section()).flatten()
        .find(|x| x.0 == target)
        .map(|(_, x)| x.0.abs() + x.1.abs())
        .unwrap()
}

type Position = (i32, i32);

struct Pattern {
    current: u32,
    position: Position,
}

impl Pattern {
    fn new() -> Pattern {
        Pattern { current: 1, position: (0, 0) }
    }

    fn next_section(&mut self) -> Vec<(u32, Position)> {
        let directions = &[(0, 1), (-1, 0), (0, -1), (1, 0)];
        let pattern_width = self.position.0 * 2 + 3;

        let mut visited = Vec::new();
        self.position.0 += 1;
        self.position.1 -= 1;

        for direction in directions {
            for _ in 1..pattern_width {
                self.position.0 += direction.0;
                self.position.1 += direction.1;
                self.current += 1;
                visited.push((self.current, self.position));
            }
        }

        visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, solve_puzzle("12"));
        assert_eq!(2, solve_puzzle("23"));
        assert_eq!(31, solve_puzzle("1024"));
    }
}
