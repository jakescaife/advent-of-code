fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-19 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let replacements: Vec<_> = input
        .lines()
        .filter_map(|x| {
            let mut split = x.split(" => ");
            Some((split.next()?, split.next()?))
        })
        .collect();

    let mut molecule = input.lines().last().unwrap().to_string();
    let mut steps = 0;

    while molecule != 'e'.to_string() {
        for (pattern, replacement) in replacements.iter() {
            steps += molecule.matches(replacement).count();
            molecule = molecule.replace(replacement, pattern);
        }
    }

    steps
}
