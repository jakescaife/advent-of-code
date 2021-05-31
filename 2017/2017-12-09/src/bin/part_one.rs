fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-09 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let mut input = input.chars();

    let mut waste = false;
    let mut depth = 0;
    let mut score = 0;

    while let Some(character) = input.next() {
        match character {
            '{' if !waste => depth += 1,
            '}' if !waste => { score += depth; depth -= 1; },
            '<' => waste = true,
            '>' => waste = false,
            '!' => { input.next(); }
            _ => continue,
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle("{}"));
        assert_eq!(6, solve_puzzle("{{{}}}"));
        assert_eq!(5, solve_puzzle("{{},{}}"));
        assert_eq!(1, solve_puzzle("{<a>,<a>,<a>,<a>}"));
        assert_eq!(9, solve_puzzle("{{<ab>},{<ab>},{<ab>},{<ab>}}"));
        assert_eq!(9, solve_puzzle("{{<!!>},{<!!>},{<!!>},{<!!>}}"));
        assert_eq!(3, solve_puzzle("{{<a!>},{<a!>},{<a!>},{<ab>}}"));
        assert_eq!(16, solve_puzzle("{{{},{},{{}}}}"));
    }
}
