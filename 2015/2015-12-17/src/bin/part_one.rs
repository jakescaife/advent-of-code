fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-17 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    let mut containers: Vec<u32> = input.lines().map(|x| x.parse().unwrap()).collect();
    containers.sort_unstable();
    combinations(&containers, 150)
}

fn combinations(containers: &[u32], target: u32) -> u32 {
    let mut total = 0;
    for (i, x) in containers.iter().enumerate() {
        total += match target.checked_sub(*x) {
            Some(x) if x == 0 => 1,
            Some(x) => combinations(&containers[i + 1..], x),
            None => break,
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let containers = vec![5, 5, 10, 15, 20];
        assert_eq!(4, combinations(&containers, 25));
    }
}
