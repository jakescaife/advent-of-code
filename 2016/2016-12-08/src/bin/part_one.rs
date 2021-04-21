use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-08 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let mut screen = Screen::new(50, 6);
    for instruction in input.lines() {
        screen.execute_instruction(instruction);
    }

    screen.count()
}

struct Screen(Vec<VecDeque<char>>);

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        let pixels = vec![VecDeque::from(vec!['.'; height]); width];
        Self(pixels)
    }

    fn execute_instruction(&mut self, instruction: &str) {
        let instruction: Vec<&str> = instruction
            .split(|x| x == ' ' || x == 'x' || x == '=')
            .collect();

        let mut parameters = instruction.iter().filter_map(|x| x.parse().ok());
        let x: usize = parameters.next().unwrap();
        let y: usize = parameters.next().unwrap();

        match instruction.as_slice() {
            ["rect", ..] => self.rectangle(x, y),
            [_, "column", ..] => self.rotate_column(x, y),
            [_, "row", ..] => self.rotate_row(x, y),
            _ => panic!("Unexpected instruction"),
        }
    }

    fn rectangle(&mut self, width: usize, height: usize) {
        for x in 0..width {
            for y in 0..height {
                self.0[x][y] = '#';
            }
        }
    }

    fn rotate_column(&mut self, column: usize, offset: usize) {
        self.0[column].rotate_right(offset);
    }

    fn rotate_row(&mut self, row: usize, offset: usize) {
        let mut values: VecDeque<_> = self.0.iter().map(|x| x[row]).collect();
        values.rotate_right(offset);

        for (column, value) in values.into_iter().enumerate() {
            self.0[column][row] = value;
        }
    }

    fn count(&self) -> usize {
        self.0.iter().flatten().filter(|&&x| x == '#').count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let instructions = "rect 3x2\n\
                            rotate column x=1 by 1\n\
                            rotate row y=0 by 4\n\
                            rotate column x=1 by 1";

        let mut screen = Screen::new(7, 3);
        for instruction in instructions.lines() {
            screen.execute_instruction(instruction);
        }

        assert_eq!(6, screen.count());
    }
}
