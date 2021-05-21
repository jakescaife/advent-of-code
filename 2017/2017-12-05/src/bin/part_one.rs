fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-05 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let mut program: Vec<i32> = input.lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut position = 0;
    let mut steps = 0;

    while let Some(offset) = program.get_mut(position as usize) {
        position += *offset;
        *offset += 1;
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(5, solve_puzzle("0\n3\n0\n1\n-3"));
    }
}
