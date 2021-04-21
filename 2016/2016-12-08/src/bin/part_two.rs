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

struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<bool>,
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![false; width * height],
        }
    }

    fn execute_instruction(&mut self, instruction: &str) {
        let instruction: Vec<&str> = instruction
            .split(|x| x == ' ' || x == '=' || x == 'x')
            .filter(|x| !x.is_empty())
            .collect();

        let params: Vec<usize> = instruction.iter().filter_map(|x| x.parse().ok()).collect();

        match instruction.as_slice() {
            ["rect", ..] => self.rectangle(params[0], params[1]),
            [_, "column", ..] => self.rotate_column(params[0], params[1]),
            [_, "row", ..] => self.rotate_row(params[0], params[1]),
            _ => panic!("Unexpected instruction"),
        }
    }

    fn rectangle(&mut self, width: usize, height: usize) {
        for column in 0..width {
            for row in 0..height {
                self.pixels[row * self.width + column] = true;
            }
        }
    }

    fn rotate_column(&mut self, column: usize, offset: usize) {
        let current_values = self.pixels.iter().skip(column).step_by(self.width);
        let rotated_values: Vec<bool> = current_values
            .cycle()
            .skip(self.height - offset)
            .copied()
            .take(self.height)
            .collect();

        for (row, value) in rotated_values.into_iter().enumerate() {
            self.pixels[row * self.width + column] = value;
        }
    }

    fn rotate_row(&mut self, row: usize, offset: usize) {
        let current_values = self.pixels.iter().skip(row * self.width).take(self.width);
        let rotated_values: Vec<bool> = current_values
            .cycle()
            .skip(self.width - offset)
            .copied()
            .take(self.width)
            .collect();

        for (column, value) in rotated_values.into_iter().enumerate() {
            self.pixels[row * self.width + column] = value;
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, &value) in self.pixels.iter().enumerate() {
            if index % self.width == 0 {
                writeln!(f)?
            }

            let value = if value { '#' } else { '.' };
            write!(f, "{}", value)?
        }

        Ok(())
    }
}
