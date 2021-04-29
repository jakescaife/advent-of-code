use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::iter;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-13 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let number = input.trim().parse().unwrap();
    breadth_first_search(number)
}

fn breadth_first_search(number: usize) -> usize {
    let start = Position { x: 1, y: 1, cost: 0 };

    let mut visited: HashSet<_> = iter::once(start.coordinates()).collect();
    let mut queue:  VecDeque<_> = iter::once(start).collect();

    while let Some(position) = queue.pop_front() {
        if position.cost == 50 { break; }

        for next in position.next_positions() {
            let coordinates = next.coordinates();

            if next.is_wall(number) || !visited.insert(coordinates) {
                continue;
            }

            queue.push_back(next);
        }
    }

    visited.len()
}

struct Position {
    x: usize,
    y: usize,
    cost: usize,
}

impl Position {
    fn is_wall(&self, favourite: usize) -> bool {
        let equation = self.x.pow(2) + 3 * self.x + 2 * self.x * self.y + self.y
            + self.y.pow(2) + favourite;

        equation.count_ones() % 2 == 1
    }

    fn next_positions(&self) -> Vec<Self> {
        let directions = &[(0, -1), (1, 0), (0, 1), (-1, 0)];
        let mut positions = Vec::new();

        for direction in directions {
            let x = match (self.x as i32 + direction.0).try_into() {
                Ok(x) => x,
                _ => continue,
            };

            let y = match (self.y as i32 + direction.1).try_into() {
                Ok(x) => x,
                _ => continue,
            };

            positions.push(Self { x, y, cost: self.cost + 1 })
        }

        positions
    }

    fn coordinates(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}
