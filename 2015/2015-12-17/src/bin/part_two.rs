fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-17 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i32 {
    let containers: Vec<i32> = input.lines().map(|x| x.parse().unwrap()).collect();
    minimum_containers(150, &containers)
}

fn minimum_containers(target: i32, containers: &[i32]) -> i32 {
    (0..)
        .map(|x| combinations(target, containers, x))
        .find(|&x| x != 0)
        .unwrap()
}

fn combinations(target: i32, containers: &[i32], depth: usize) -> i32 {
    if target == 0 {
        return 1;
    } else if target < 0 || containers.is_empty() || depth == 0 {
        return 0;
    }

    let (x, tail) = containers.split_first().unwrap();
    combinations(target - x, tail, depth - 1) + combinations(target, tail, depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, minimum_containers(25, &[5, 5, 10, 15, 20]));
    }
}
