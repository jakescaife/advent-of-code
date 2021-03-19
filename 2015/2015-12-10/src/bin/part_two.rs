fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-10 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let input = input.trim();
    (0..50)
        .fold(input.to_string(), |s, _| next_sequence(&s))
        .len()
}

fn next_sequence(sequence: &str) -> String {
    let mut sequences = Vec::new();
    let mut current_sequence = String::new();

    for c in sequence.chars() {
        if current_sequence.is_empty() || current_sequence.contains(c) {
            current_sequence.push(c);
        } else {
            sequences.push(current_sequence.clone());
            current_sequence = String::from(c);
        }
    }

    sequences.push(current_sequence);
    sequences
        .iter()
        .map(|x| format!("{}{}", x.len(), &x[..1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(
            "312211".to_string(),
            (0..5).fold("1".to_string(), |s, _| next_sequence(&s))
        );
    }
}
