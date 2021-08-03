fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let steps: usize = input.trim().parse().unwrap();
    let mut index = 0;
    let mut solution = 0;

    for length in 1..50000000 {
        index = 1 + (index + steps) % length;
        if index == 1 { solution = length }
    }

    println!("AOC 2017-17 Part Two: {}", solution);
}
