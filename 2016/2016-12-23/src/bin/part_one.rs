use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-23 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let mut computer = Computer::default();
    *computer.memory.register('a') = 7;
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
        let mut instructions: Vec<Instruction> = instructions.lines()
            .map(Instruction::from)
            .collect();

        while let Some(instruction) = instructions.get(self.counter).copied() {
            match instruction {
                Instruction::Copy(x, y) => self.memory.copy(x, y),
                Instruction::Increment(x) => self.memory.increment(x,  1),
                Instruction::Decrement(x) => self.memory.increment(x, -1),
                Instruction::JumpNotZero(x, y) => self.counter_jump(x, y),
                Instruction::Toggle(x) => self.toggle(x, &mut instructions),
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
            Parameter::Register(x) => self.memory.value(x).unwrap(),
        };

        let offset = match value { 0 => 1, _ => offset };

        self.counter = (self.counter as i32 + offset) as usize;
    }

    fn toggle(&self, parameter: Parameter, instructions: &mut [Instruction]) {
        let offset = match parameter {
            Parameter::Value(x) => x,
            Parameter::Register(x) => self.memory.value(x).unwrap(),
        };

        let offset = (self.counter as i32 + offset) as usize;

        if let Some(instruction) = instructions.get_mut(offset) {
            *instruction = instruction.toggle();
        }
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
    Toggle(Parameter),
}

impl Instruction {
    fn toggle(self) -> Self {
        match self {
            Self::Copy(x, y) => Self::JumpNotZero(x, y),
            Self::Increment(x) => Self::Decrement(x),
            Self::Decrement(x) => Self::Increment(x),
            Self::JumpNotZero(x, y) => Self::Copy(x, y),
            Self::Toggle(x) => Self::Increment(x),
        }
    }
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        let slice: Vec<&str> = instruction.split_whitespace().collect();
        match slice[0] {
            "cpy" => Self::Copy(slice[1].into(), slice[2].into()),
            "inc" => Self::Increment(slice[1].into()),
            "dec" => Self::Decrement(slice[1].into()),
            "jnz" => Self::JumpNotZero(slice[1].into(), slice[2].into()),
            "tgl" => Self::Toggle(slice[1].into()),
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
        let instructions = "cpy 2 a\ntgl a\ntgl a\ntgl a\ncpy 1 a\ndec a\ndec a";
        assert_eq!(3, solve_puzzle(instructions));
    }
}
