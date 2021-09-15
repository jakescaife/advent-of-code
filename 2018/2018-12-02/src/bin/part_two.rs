fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2018-02 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    for first in input.lines() {
        for second in input.lines() {
            let common: String = first.chars().zip(second.chars())
                .filter_map(|x| (x.0 == x.1).then(|| x.0))
                .collect();
            if common.len() == first.len() - 1 {
                return common;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let example = "abcde\n\
                       fghij\n\
                       klmno\n\
                       pqrst\n\
                       fguij\n\
                       axcye\n\
                       wvxyz";
        assert_eq!("fgij", solve_puzzle(example).as_str());
    }
}
