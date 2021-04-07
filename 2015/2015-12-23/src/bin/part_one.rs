use std::collections::HashMap;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-23 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let mut computer = Computer::from(input);
    computer.execute_program();
    computer.memory.value('b').unwrap()
}

struct Computer {
    memory: Memory,
    counter: InstructionCounter,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn execute_program(&mut self) {
        while let Some(instruction) = self.instructions.get(*self.counter) {
            match instruction {
                Instruction::HLF(r) => self.memory.half(*r),
                Instruction::TPL(r) => self.memory.triple(*r),
                Instruction::INC(r) => self.memory.increment(*r),
                Instruction::JMP(o) => self.counter.jump(*o),
                Instruction::JIE(r, o) => self.counter.jump_if_even(*o, self.memory.value(*r)),
                Instruction::JIO(r, o) => self.counter.jump_if_one(*o, self.memory.value(*r)),
            }

            let has_not_jumped = matches!(
                instruction,
                Instruction::HLF(_) | Instruction::TPL(_) | Instruction::INC(_)
            );

            if has_not_jumped {
                self.counter.increment();
            }
        }
    }
}

impl From<&str> for Computer {
    fn from(s: &str) -> Self {
        Self {
            memory: Memory::default(),
            counter: InstructionCounter::default(),
            instructions: s.lines().map(Instruction::from).collect(),
        }
    }
}

#[derive(Default)]
struct Memory {
    registers: HashMap<char, usize>,
}

impl Memory {
    fn half(&mut self, register: char) {
        *self.register(register) /= 2;
    }

    fn triple(&mut self, register: char) {
        *self.register(register) *= 3;
    }

    fn increment(&mut self, register: char) {
        *self.register(register) += 1;
    }

    fn register(&mut self, register: char) -> &mut usize {
        self.registers.entry(register).or_insert(0)
    }

    fn value(&self, register: char) -> Option<usize> {
        self.registers.get(&register).copied()
    }
}

#[derive(Default)]
struct InstructionCounter {
    counter: usize,
}

impl InstructionCounter {
    fn increment(&mut self) {
        self.counter += 1;
    }

    fn jump(&mut self, offset: i32) {
        self.counter = (self.counter as i32 + offset) as usize;
    }

    fn jump_if_even(&mut self, offset: i32, value: Option<usize>) {
        if value.map(|x| x & 1) == Some(0) {
            self.jump(offset);
        } else {
            self.increment();
        }
    }

    fn jump_if_one(&mut self, offset: i32, value: Option<usize>) {
        if value == Some(1) {
            self.jump(offset);
        } else {
            self.increment();
        }
    }
}

impl std::ops::Deref for InstructionCounter {
    type Target = usize;
    fn deref(&self) -> &usize {
        &self.counter
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    HLF(char),
    TPL(char),
    INC(char),
    JMP(i32),
    JIE(char, i32),
    JIO(char, i32),
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let s = s.replace(",", "");
        let s: Vec<_> = s.split_whitespace().collect();
        match s.as_slice() {
            ["hlf", r] => Self::HLF(r.chars().next().unwrap()),
            ["tpl", r] => Self::TPL(r.chars().next().unwrap()),
            ["inc", r] => Self::INC(r.chars().next().unwrap()),
            ["jmp", o] => Self::JMP(o.parse().unwrap()),
            ["jie", r, o] => Self::JIE(r.chars().next().unwrap(), o.parse().unwrap()),
            ["jio", r, o] => Self::JIO(r.chars().next().unwrap(), o.parse().unwrap()),
            _ => panic!("Unexpected instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let mut computer = Computer::from("inc a\njio a, +2\ntpl a\ninc a");
        computer.execute_program();
        assert_eq!(Some(2), computer.memory.value('a'));
    }
}
