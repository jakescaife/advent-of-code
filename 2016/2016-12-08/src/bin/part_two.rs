use std::collections::VecDeque;
use std::fmt;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-08 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let mut screen = Screen::new(50, 6);
    for instruction in input.lines() {
        screen.execute_instruction(instruction);
    }

    screen.to_string()
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
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (width, height) = (self.0.len(), self.0[0].len());
        for y in 0..height {
            writeln!(f)?;
            for x in 0..width {
                write!(f, "{}", self.0[x][y])?;
            }
        }
        Ok(())
    }
}
