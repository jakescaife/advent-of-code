use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-08 Part Two: {}", solve_puzzle(&input));
}

struct Instruction<'a> {
    register: &'a str,
    increase: &'a str,
    amount: i32,
    dependency: &'a str,
    operation: &'a str,
    condition: i32,
}

fn parse_instruction(s: &str) -> Instruction {
    let instruction: Vec<_> = s.split_whitespace().collect();
    Instruction {
        register:   instruction[0],
        increase:   instruction[1],
        amount:     instruction[2].parse().unwrap(),
        dependency: instruction[4],
        operation:  instruction[5],
        condition:  instruction[6].parse().unwrap(),
    }
}

fn solve_puzzle(input: &str) -> i32 {
    let mut registers = HashMap::new();
    let mut maximum = 0;

    for instruction in input.lines().map(parse_instruction) {
        let dependency = *registers.entry(instruction.dependency).or_insert(0);
        let valid_condition = match instruction.operation {
            ">"  => dependency >  instruction.condition,
            "<"  => dependency <  instruction.condition,
            ">=" => dependency >= instruction.condition,
            "<=" => dependency <= instruction.condition,
            "!=" => dependency != instruction.condition,
            "==" => dependency == instruction.condition,
            _ => panic!("Unexpected operation"),
        };

        if valid_condition {
            let register = registers.entry(instruction.register).or_insert(0);
            match instruction.increase {
                "inc" => *register += instruction.amount,
                "dec" => *register -= instruction.amount,
                _ => panic!("Unexpected increment"),
            };

            maximum = maximum.max(*register);
        }
    }

    maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let example = "b inc 5 if a > 1\n\
                       a inc 1 if b < 5\n\
                       c dec -10 if a >= 1\n\
                       c inc -20 if c == 10";

        assert_eq!(10, solve_puzzle(example));
    }
}
