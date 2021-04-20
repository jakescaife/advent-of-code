fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-07 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    input.lines().filter(|x| supports_tls(x)).count()
}

fn supports_tls(ip: &str) -> bool {
    let sections: Vec<_> = ip
        .split_inclusive(|x| x == '[' || x == ']')
        .filter(|x| contains_abba(x))
        .collect();

    sections.iter().all(|x| !x.ends_with(']')) && !sections.is_empty()
}

fn contains_abba(section: &str) -> bool {
    let characters: Vec<_> = section.chars().collect();
    characters
        .windows(4)
        .any(|x| x[0] != x[1] && x[0] == x[3] && x[1] == x[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(1, solve_puzzle("abba[mnop]qrst"));
        assert_eq!(0, solve_puzzle("abcd[bddb]xyyx"));
        assert_eq!(0, solve_puzzle("aaaa[qwer]tyui"));
        assert_eq!(1, solve_puzzle("ioxxoj[asdfgh]zxcvbn"));
    }
}
