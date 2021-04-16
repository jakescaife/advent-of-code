fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2015-05 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let input = input.trim();
    let mut password = vec![None; 8];

    for (position, value) in (0..).filter_map(|x| next_character(input, x)) {
        if password[position].is_some() {
            continue;
        }

        password[position] = Some(value);

        if !password.contains(&None) {
            break;
        }
    }

    password.iter().filter_map(|&x| x).collect()
}

fn next_character(word: &str, number: usize) -> Option<(usize, char)> {
    let key = format!("{}{}", word, number);
    let hash = format!("{:?}", md5::compute(key));

    let position = hash.chars().nth(5)?.to_digit(10).filter(|&x| x < 8)?;
    let value = hash.chars().nth(6)?;

    hash.starts_with("00000").then(|| (position as usize, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!("05ace8e3", &solve_puzzle("abc"));
    }
}
