fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-11 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let destination = input.trim().split(',').fold((0, 0, 0), next_hex);
    (destination.0.abs() + destination.1.abs() + destination.2.abs()) / 2
}

fn next_hex(mut position: (i32, i32, i32), direction: &str) -> (i32, i32, i32) {
    match direction {
        "n"  => { position.1 += 1; position.2 -= 1 },
        "s"  => { position.1 -= 1; position.2 += 1 },
        "ne" => { position.0 += 1; position.2 -= 1 },
        "sw" => { position.0 -= 1; position.2 += 1 },
        "nw" => { position.0 -= 1; position.1 += 1 },
        "se" => { position.0 += 1; position.1 -= 1 },
        other => panic!("Unexpected direction: {}", other),
    }

    position
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, solve_puzzle("ne,ne,ne"));
        assert_eq!(0, solve_puzzle("ne,ne,sw,sw"));
        assert_eq!(2, solve_puzzle("ne,ne,s,s"));
        assert_eq!(3, solve_puzzle("se,sw,se,sw,sw"));
    }
}
