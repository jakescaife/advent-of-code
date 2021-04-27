use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-12 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let mut computer = Computer::default();
    computer.execute_instructions(input);
    computer.memory.value('a').unwrap()
}

#[derive(Default)]
struct Computer {
    counter: usize,
    memory: Memory,
}

impl Computer {
    fn execute_instructions(&mut self, instructions: &str) {
        let instructions: Vec<Instruction> = instructions.lines()
            .map(Instruction::from)
            .collect();

        while let Some(instruction) = instructions.get(self.counter) {
            match instruction {
                Instruction::Copy(x, y) => self.memory.copy(*x, *y),
                Instruction::Increment(x) => self.memory.increment(*x,  1),
                Instruction::Decrement(x) => self.memory.increment(*x, -1),
                Instruction::JumpNotZero(x, y) => self.counter_jump(*x, *y),
            }

            if !matches!(instruction, Instruction::JumpNotZero(_,_)) {
                self.counter += 1;
            }
        }
    }

    fn counter_jump(&mut self, parameter: Parameter, offset: Parameter) {
        let value = match parameter {
            Parameter::Register(x) => *self.memory.register(x),
            Parameter::Value(x) => x,
        };

        let offset = match offset {
            Parameter::Value(x) => x,
            Parameter::Register(_) => panic!("Attempt to use register as jump offset"),
        };

        let offset = match value { 0 => 1, _ => offset };

        self.counter = (self.counter as i32 + offset) as usize;
    }
}

#[derive(Default)]
struct Memory {
    memory: HashMap<char, i32>,
}

impl Memory {
    fn copy(&mut self, first: Parameter, second: Parameter) {
        let value = match first {
            Parameter::Register(x) => self.value(x).unwrap(),
            Parameter::Value(x) => x,
        };

        match second {
            Parameter::Register(x) => *self.register(x) = value,
            Parameter::Value(_) => panic!("Cannot copy to value"),
        }
    }

    fn increment(&mut self, parameter: Parameter, amount: i32) {
        match parameter {
            Parameter::Register(x) => *self.register(x) += amount,
            Parameter::Value(_) => panic!("Attempt to modify non-register"),
        }
    }

    fn value(&self, register: char) -> Option<i32> {
        self.memory.get(&register).copied()
    }

    fn register(&mut self, register: char) -> &mut i32 {
        self.memory.entry(register).or_default()
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Copy(Parameter, Parameter),
    Increment(Parameter),
    Decrement(Parameter),
    JumpNotZero(Parameter, Parameter),
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        let slice: Vec<&str> = instruction.split_whitespace().collect();
        match slice[0] {
            "cpy" => Self::Copy(slice[1].into(), slice[2].into()),
            "inc" => Self::Increment(slice[1].into()),
            "dec" => Self::Decrement(slice[1].into()),
            "jnz" => Self::JumpNotZero(slice[1].into(), slice[2].into()),
            _ => panic!("Unexpected instruction"),
        }
    }
}

#[derive(Copy, Clone)]
enum Parameter {
    Register(char),
    Value(i32),
}

impl From<&str> for Parameter {
    fn from(parameter: &str) -> Self {
        match parameter.parse::<i32>() {
            Ok(x) => Self::Value(x),
            Err(_) => Self::Register(parameter.chars().next().unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let instructions = "cpy 41 a\ninc a\ninc a\ndec a\njnz a 2\ndec a";
        assert_eq!(42, solve_puzzle(instructions));
    }
}
