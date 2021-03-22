use std::collections::HashSet;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-11 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> String {
    let mut password: Vec<char> = input.trim().chars().collect();

    while !valid_password(&password) {
        next_password(&mut password);
    }

    next_password(&mut password);

    while !valid_password(&password) {
        next_password(&mut password);
    }

    password.iter().collect()
}

fn next_password(password: &mut [char]) -> Option<()> {
    let (last, remaining) = password.split_last_mut()?;
    let next = next_character(*last)?;

    if next == 'a' {
        next_password(remaining);
    }

    *last = next;
    Some(())
}

fn next_character(c: char) -> Option<char> {
    match std::char::from_u32(c as u32 + 1) {
        Some('{') => Some('a'),
        Some('i') => Some('j'),
        Some('l') => Some('m'),
        Some('o') => Some('p'),
        x => x,
    }
}

fn valid_password(password: &[char]) -> bool {
    has_sequence(password)
        && has_pairs(password)
        && password.iter().all(|x| !['i', 'o', 'l'].contains(x))
}

fn has_sequence(password: &[char]) -> bool {
    password
        .windows(3)
        .any(|x| x[0] as u32 + 1 == x[1] as u32 && x[0] as u32 + 2 == x[2] as u32)
}

fn has_pairs(password: &[char]) -> bool {
    let pairs: HashSet<_> = password.windows(2).filter(|x| x[0] == x[1]).collect();
    pairs.len() >= 2
}
