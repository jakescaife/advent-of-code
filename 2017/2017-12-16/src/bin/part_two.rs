use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-16 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    let mut visited = HashMap::new();

    for iteration in 0..1000000000 {
        visited.insert(programs.clone(), iteration);
        perform_dance(&mut programs, input);
        if let Some(repeat) = visited.get(&programs) {
            let cycle = iteration  - repeat + 1;
            let index = (999999999 - iteration) % cycle;
            let remaining = visited.iter().find(|x| index == *x.1).unwrap();
            return remaining.0.iter().collect();
        }
    }

    unreachable!()
}

fn perform_dance(programs: &mut [char], dance_moves: &str) {
    for dance_move in dance_moves.trim().split(',') {
        match dance_move.split_at(1) {
            ("x", details) => exchange(programs, details),
            ("s", details) => spin(programs, details),
            ("p", details) => partner(programs, details),
            _ => panic!("Unrecognised dance move")
        }
    }
}

fn exchange(programs: &mut [char], details: &str) {
    let details: Vec<usize> = details.split('/')
        .map(|x| x.parse().unwrap())
        .collect();
    programs.swap(details[0], details[1]);
}

fn spin(programs: &mut [char], details: &str) {
    let rotation_index: usize = details.parse().unwrap();
    programs.rotate_right(rotation_index);
}

fn partner(programs: &mut [char], details: &str) {
    let details: Vec<usize> = details.chars()
        .filter_map(|x| programs.iter().position(|&z| z == x))
        .collect();
    programs.swap(details[0], details[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinning_dance() {
        let mut programs: Vec<char> = "abcde".chars().collect();
        spin(&mut programs, "1");
        assert_eq!(vec!['e','a','b','c','d'], programs);
    }

    #[test]
    fn exchange_dance() {
        let mut programs: Vec<char> = "eabcd".chars().collect();
        exchange(&mut programs, "3/4");
        assert_eq!(vec!['e','a','b','d','c'], programs);
    }

    #[test]
    fn partner_dance() {
        let mut programs: Vec<char> = "eabdc".chars().collect();
        partner(&mut programs, "e/b");
        assert_eq!(vec!['b','a','e','d','c'], programs);
    }
}
