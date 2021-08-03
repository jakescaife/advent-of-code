fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-17 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let steps: usize = input.trim().parse().unwrap();
    let mut index = 0;
    let mut output = 0;

    for length in 1..50000000 {
        index = (index + steps) % length + 1;
        if index == 1 { output = length }
    }

    output
}
