fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-20 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let maximum: usize = input.trim().parse().unwrap();
    let mut houses = vec![0; maximum / 10];

    for elf in 1..maximum / 10 {
        for house in (elf..maximum / 10).step_by(elf) {
            houses[house] += elf * 10;
        }
    }

    houses.iter().position(|&x| x >= maximum).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(6, solve_puzzle("120"));
    }
}
