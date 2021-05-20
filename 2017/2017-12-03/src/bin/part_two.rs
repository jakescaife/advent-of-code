use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-03 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let target = input.trim().parse().unwrap();
    let mut pattern = Pattern::new();

    (0..).map(|_| pattern.next_section()).flatten()
        .find(|x| *x >=  target)
        .unwrap()
}

type Position = (i32, i32);

struct Pattern {
    position: Position,
    visited: HashMap<Position, u32>,
}

impl Pattern {
    fn new() -> Pattern {
        let mut new_pattern = Pattern {
            position: (0, 0),
            visited: HashMap::new(),
        };

        new_pattern.visited.insert((0, 0), 1);
        new_pattern
    }

    fn next_section(&mut self) -> Vec<u32> {
        let directions = &[(0, 1), (-1, 0), (0, -1), (1, 0)];
        let pattern_width = self.position.0 * 2 + 3;

        let mut values = Vec::new();
        self.position.0 += 1;
        self.position.1 -= 1;

        for direction in directions {
            for _ in 1..pattern_width {
                self.position.0 += direction.0;
                self.position.1 += direction.1;

                let next_value = self.calculate_value();
                self.visited.insert(self.position, next_value);
                values.push(next_value);
            }
        }

        values
    }

    fn calculate_value(&mut self) -> u32 {
        let directions = &[
            (-1,  1), (0,  1), (1,  1),
            (-1,  0), (0,  0), (1,  0),
            (-1, -1), (0, -1), (1, -1),
        ];

        directions.iter()
            .map(|x| (self.position.0 + x.0, self.position.1 + x.1))
            .map(|x| self.visited.get(&x).unwrap_or(&0))
            .sum()
    }
}
