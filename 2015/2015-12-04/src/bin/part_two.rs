fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-04 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let input = input.trim();
    (0..).find(|x| has_zeroes(input, *x)).unwrap()
}

fn has_zeroes(word: &str, number: usize) -> bool {
    let digest = md5::compute(format!("{}{}", word, number));
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}
