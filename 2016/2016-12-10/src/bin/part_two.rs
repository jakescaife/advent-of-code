use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-10 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let (setup, instructions): (Vec<_>, Vec<_>) = input.lines()
        .partition(|x| x.starts_with("value"));

    let mut bots = HashMap::new();
    let mut outputs = HashMap::new();

    for instruction in instructions {
        let values: Vec<_> = instruction.split_whitespace().collect();

        let min_target = Target::new(values[5], values[6]);
        let max_target = Target::new(values[10], values[11]);
        let bot_name: usize = values[1].parse().unwrap();

        bots.insert(bot_name, Bot::new(min_target, max_target));
    }

    for instruction in setup {
        let values: Vec<usize> = instruction.split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        bots.get_mut(&values[1]).unwrap().give(values[0]);
    }

    while let Some((name, bot)) = bots.iter().find(|x| x.1.can_transfer()) {
        let (min_target, min_val) = bot.fetch_min();
        let (max_target, max_val) = bot.fetch_max();

        let name = *name;

        let mut transfer = |target, value| match target {
            Target::Bot(x) => { bots.get_mut(&x).unwrap().give(value); },
            Target::Output(x) => { outputs.insert(x, value); },
        };

        transfer(min_target, min_val);
        transfer(max_target, max_val);
        bots.remove(&name);
    }

    let output0 = outputs.get(&0).unwrap();
    let output1 = outputs.get(&1).unwrap();
    let output2 = outputs.get(&2).unwrap();

    output0 * output1 * output2
}

struct Bot {
    min_target: Target,
    max_target: Target,
    values: Vec<usize>,
}

impl Bot {
    fn new(min_target: Target, max_target: Target) -> Self {
        Self { min_target, max_target, values: vec![] }
    }

    fn give(&mut self, value: usize) {
        self.values.push(value)
    }

    fn can_transfer(&self) -> bool {
        self.values.len() == 2
    }

    fn fetch_min(&self) -> (Target, usize) {
        let min = self.values.iter().min().unwrap();
        (self.min_target, *min)
    }

    fn fetch_max(&self) -> (Target, usize) {
        let max = self.values.iter().max().unwrap();
        (self.max_target, *max)
    }
}

#[derive(Copy, Clone)]
enum Target {
    Bot(usize),
    Output(usize),
}

impl Target {
    fn new(target_type: &str, id: &str) -> Self {
        let id: usize = id.parse().unwrap();
        match target_type {
            "bot" => Self::Bot(id),
            "output" => Self::Output(id),
            _ => panic!("Unexpected output type"),
        }
    }
}
