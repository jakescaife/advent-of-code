use std::collections::VecDeque;
use std::fmt;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-21 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> Password {
    input.lines().fold(Password::new("abcdefgh"), |mut s, x| { s.execute(x); s })
}

struct Password {
    characters: VecDeque<char>,
}

impl Password {
    fn new(password: &str) -> Self {
        Self { characters: password.chars().collect() }
    }

    fn execute(&mut self, task: &str) {
        let task: Vec<_> = task.split_whitespace().collect();

        let n = |x: &str| x.parse().unwrap();
        let c = |x: &str| x.chars().next().unwrap();

        match task.as_slice() {
            ["swap", "position", x, .., y] => self.swap(n(x), n(y)),
            ["swap", "letter", x, .., y] => self.swap_letter(c(x), c(y)),
            ["rotate", x, y, _] => self.rotate(c(x), n(y)),
            ["rotate", .., x] => self.rotate_letter(c(x)),
            ["reverse", _, x, _, y] => self.reverse(n(x), n(y)),
            ["move", _, x, .., y] => self.remove(n(x), n(y)),
            _ => panic!("Unexpected instruction"),
        }
    }

    fn swap(&mut self, x: usize, y: usize) {
        self.characters.swap(x, y);
    }

    fn swap_letter(&mut self, x: char, y: char) {
        let x = self.characters.iter().position(|&c| x == c).unwrap();
        let y = self.characters.iter().position(|&c| y == c).unwrap();

        self.swap(x, y);
    }

    fn reverse(&mut self, x: usize, y: usize) {
        self.characters.make_contiguous()[x..=y].reverse()
    }

    fn rotate(&mut self, direction: char, steps: usize) {
        match direction {
            'l' => self.characters.rotate_left(steps),
            'r' => self.characters.rotate_right(steps),
            _ => panic!("Unexpected rotation direction"),
        }
    }

    fn remove(&mut self, origin: usize, target: usize) {
        let character = self.characters.remove(origin).unwrap();
        self.characters.insert(target, character);
    }

    fn rotate_letter(&mut self, letter: char) {
        let mut steps = self.characters.iter().position(|&x| x == letter).unwrap();

        steps += if steps > 3 { 2 } else { 1 };
        steps %= self.characters.len();

        self.rotate('r', steps);
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.characters.iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        let mut password = Password::new("abcde");

        password.execute("swap position 4 with position 0");
        assert_eq!("ebcda", &format!("{}", password));

        password.execute("swap letter d with letter b");
        assert_eq!("edcba", &format!("{}", password));

        password.execute("reverse positions 0 through 4");
        assert_eq!("abcde", &format!("{}", password));

        password.execute("rotate left 1 step");
        assert_eq!("bcdea", &format!("{}", password));

        password.execute("move position 1 to position 4");
        assert_eq!("bdeac", &format!("{}", password));

        password.execute("move position 3 to position 0");
        assert_eq!("abdec", &format!("{}", password));

        password.execute("rotate based on position of letter b");
        assert_eq!("ecabd", &format!("{}", password));

        password.execute("rotate based on position of letter d");
        assert_eq!("decab", &format!("{}", password));
    }
}
