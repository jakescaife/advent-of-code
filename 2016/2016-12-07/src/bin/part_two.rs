fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-07 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().filter(|x| supports_ssl(x)).count()
}

fn supports_ssl(ip: &str) -> bool {
    let (hypernet, supernet): (Vec<_>, Vec<_>) = ip
        .split_inclusive(|x| x == '[' || x == ']')
        .partition(|x| x.ends_with(']'));

    let supernet: Vec<Vec<char>> = supernet.into_iter()
        .map(|x| x.chars().collect())
        .collect();

    supernet.iter()
        .flat_map(|x| x.windows(3))
        .filter(|x| x[0] != x[1] && x[0] == x[2])
        .map(|x| format!("{}{}{}", x[1], x[0], x[1]))
        .any(|x| hypernet.iter().any(|y| y.contains(&x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle("aba[bab]xyz"));
        assert_eq!(0, solve_puzzle("xyx[xyx]xyx"));
        assert_eq!(1, solve_puzzle("aaa[kek]eke"));
        assert_eq!(1, solve_puzzle("zazbz[bzb]cdb"));
    }
}
