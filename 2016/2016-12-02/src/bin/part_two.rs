fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-02 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let keypad = [
        ['-', '-', '-', '-', '-', '-', '-'],
        ['-', '-', '-', '1', '-', '-', '-'],
        ['-', '-', '2', '3', '4', '-', '-'],
        ['-', '5', '6', '7', '8', '9', '-'],
        ['-', '-', 'A', 'B', 'C', '-', '-'],
        ['-', '-', '-', 'D', '-', '-', '-'],
        ['-', '-', '-', '-', '-', '-', '-'],
    ];

    let mut position: (usize, usize) = (3, 1);
    let mut password = String::new();

    for line in input.lines() {
        for instruction in line.chars() {
            let next_position = match instruction {
                'U' => (position.0 - 1, position.1),
                'R' => (position.0, position.1 + 1),
                'D' => (position.0 + 1, position.1),
                'L' => (position.0, position.1 - 1),
                _ => panic!("Unexpected position"),
            };

            if keypad[next_position.0][next_position.1] != '-' {
                position = next_position;
            }
        }

        let keypad_character = keypad[position.0][position.1];
        password.push(keypad_character);
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let instructions = "ULL\nRRDDD\nLURDL\nUUUUD";
        assert_eq!("5DB3".to_string(), solve_puzzle(instructions));
    }
}
