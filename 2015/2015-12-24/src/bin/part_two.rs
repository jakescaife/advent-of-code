fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-24 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i64 {
    let packages: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
    let weight: i64 = packages.iter().sum();

    for compartments in 0.. {
        let entanglement = combinations(weight / 4, &packages, compartments, 1);
        if entanglement < i64::MAX {
            return entanglement;
        }
    }

    unreachable!()
}

fn combinations(target: i64, packages: &[i64], depth: usize, entanglement: i64) -> i64 {
    if target == 0 {
        return entanglement;
    } else if target < 0 || packages.is_empty() || depth == 0 {
        return i64::MAX;
    }

    let (head, tail) = packages.split_first().unwrap();
    let with_head = combinations(target - head, tail, depth - 1, entanglement * head);
    let without_head = combinations(target, tail, depth, entanglement);

    with_head.min(without_head)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        assert_eq!(44, solve_puzzle("1\n2\n3\n4\n5\n7\n8\n9\n10\n11"));
    }
}
