fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-16 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
    for dance_move in input.trim().split(',') {
        match dance_move.split_at(1) {
            ("x", details) => exchange(&mut programs, details),
            ("s", details) => spin(&mut programs, details),
            ("p", details) => partner(&mut programs, details),
            _ => panic!("Unrecognised dance move")
        }
    }

    programs.into_iter().collect()
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
