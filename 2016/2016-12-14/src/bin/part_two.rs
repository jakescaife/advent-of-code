use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-14 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> isize {
    let input = input.trim();

    let mut possible = HashMap::new();
    let mut hashes = Vec::new();
    let mut number = 0;

    while !(hashes.len() > 63 && hashes[63] + 1000 < number) {
        let plaintext = format!("{}{}", input, number);
        let mut hash = format!("{:?}", md5::compute(plaintext));
        possible.remove(&(number - 1000));

        for _ in 0..2016 {
            hash = format!("{:x}", md5::compute(hash));
        }

        if let Some(pattern) = sequence(&hash, 5) {
            let valid_keys: Vec<_> = possible.iter()
                .filter(|x| x.1 == &pattern)
                .map(|x| *x.0)
                .collect();

            for key in valid_keys {
                possible.remove(&key);
                hashes.push(key);
            }

            hashes.sort_unstable();
        }

        if let Some(pattern) = sequence(&hash, 3) {
            possible.insert(number, pattern);
        }

        number += 1;
    }

    hashes[63]
}

fn sequence(hash: &str, length: usize) -> Option<char> {
    hash.as_bytes()
        .windows(length)
        .find(|x| x.iter().all(|&y| y == x[0]))
        .map(|x| x[0] as char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(22551, solve_puzzle("abc"));
    }
}
