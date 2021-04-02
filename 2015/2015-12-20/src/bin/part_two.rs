fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-20 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let maximum: usize = input.trim().parse().unwrap();
    let mut houses = vec![0; maximum / 11];

    for elf in 1..maximum / 11 {
        for house in (elf..maximum / 11).step_by(elf).take(50) {
            houses[house] += elf * 11;
        }
    }

    houses.iter().position(|&x| x >= maximum).unwrap()
}
