use std::collections::HashMap;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-07 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u16 {
    let mut circuit = Circuit::from(input);

    circuit.cache_value("b", "3176");
    circuit.solve("a")
}

struct Circuit {
    wires: HashMap<String, Wire>,
}

#[derive(Clone)]
struct Wire {
    task: String,
    cache: Option<u16>,
}

impl Circuit {
    fn solve(&mut self, id: &str) -> u16 {
        if let Ok(x) = id.parse() {
            return x;
        }

        let wire = self.wires[id].clone();

        if let Some(x) = wire.cache {
            return x;
        }

        self.cache_value(id, &wire.task)
    }

    fn cache_value(&mut self, id: &str, task: &str) -> u16 {
        let task: Vec<&str> = task.split_whitespace().collect();
        let value = match task.as_slice() {
            [a] => self.solve(a),
            [a, "OR", b] => self.solve(a) | self.solve(b),
            [a, "AND", b] => self.solve(a) & self.solve(b),
            [a, "LSHIFT", b] => self.solve(a) << self.solve(b),
            [a, "RSHIFT", b] => self.solve(a) >> self.solve(b),
            ["NOT", a] => !self.solve(a),
            _ => panic!("Unexpected wire task."),
        };

        self.wires.get_mut(id).unwrap().cache = Some(value);
        value
    }
}

impl From<&str> for Circuit {
    fn from(s: &str) -> Self {
        let wires = s.lines().fold(HashMap::new(), |mut map, x| {
            let x: Vec<&str> = x.rsplit(" -> ").collect();
            map.insert(x[0].to_string(), Wire::from(x[1]));
            map
        });

        Self { wires }
    }
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        Self {
            task: s.to_string(),
            cache: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        let instructions = "123 -> x\n\
                            456 -> y\n\
                            x AND y -> d\n\
                            x OR y -> e\n\
                            x LSHIFT 2 -> f\n\
                            y RSHIFT 2 -> g\n\
                            NOT x -> h\n\
                            NOT y -> i";

        let mut circuit = Circuit::from(instructions);
        assert_eq!(circuit.solve("d"), 72);
        assert_eq!(circuit.solve("e"), 507);
        assert_eq!(circuit.solve("f"), 492);
        assert_eq!(circuit.solve("g"), 114);
        assert_eq!(circuit.solve("h"), 65412);
        assert_eq!(circuit.solve("i"), 65079);
        assert_eq!(circuit.solve("x"), 123);
        assert_eq!(circuit.solve("y"), 456);
    }
}
