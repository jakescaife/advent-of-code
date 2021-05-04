use std::collections::VecDeque;
use std::convert::TryInto;
use std::iter;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-17 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    breadth_first_search(input.trim())
}

fn breadth_first_search(password: &str) -> usize {
    let initial = State { x: 0, y: 0, path: String::new() };
    let mut queue: VecDeque<_> = iter::once(initial).collect();
    let mut length = 0;

    while let Some(state) = queue.pop_front() {
        if state.x == 3 && state.y == 3 {
            length = length.max(state.path.len());
            continue;
        }

        queue.append(&mut state.next_states(password));
    }

    length
}

#[derive(Debug)]
struct State {
    x: usize,
    y: usize,
    path: String,
}

impl State {
    fn next_states(&self, password: &str) -> VecDeque<Self> {
        let directions = &[(0, -1, 'U'), (0, 1, 'D'), (-1, 0, 'L'), (1, 0, 'R')];
        let open_doors = &['b', 'c', 'd', 'e', 'f'];

        let password = format!("{}{}", password, self.path);
        let hash = format!("{:x}", md5::compute(password));

        let mut states = VecDeque::new();

        for (direction, character) in directions.iter().zip(hash.chars()) {
            if !open_doors.contains(&character) {
                continue;
            }

            let x = match (self.x as i32 + direction.0).try_into() {
                Ok(x) if x < 4 => x,
                _ => continue,
            };

            let y = match (self.y as i32 + direction.1).try_into() {
                Ok(x) if x < 4 => x,
                _ => continue,
            };

            let path = format!("{}{}", self.path, direction.2);
            states.push_back(Self { x, y, path });
        }

        states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(370, solve_puzzle("ihgpwlah"));
        assert_eq!(492, solve_puzzle("kglvqrro"));
        assert_eq!(830, solve_puzzle("ulqzkmiv"));
    }
}
