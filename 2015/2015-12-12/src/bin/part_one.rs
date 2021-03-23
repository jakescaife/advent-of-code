use serde_json::Value;

fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-12 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i64 {
    let value: Value = serde_json::from_str(input).unwrap();
    count_numbers(&value)
}

fn count_numbers(value: &Value) -> i64 {
    match value {
        Value::Number(x) => x.as_i64().unwrap(),
        Value::Array(x) => x.iter().map(|x| count_numbers(x)).sum(),
        Value::Object(x) => x.values().map(|x| count_numbers(x)).sum(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(6, solve_puzzle("[1,2,3]"));
        assert_eq!(6, solve_puzzle("{\"a\":2, \"b\":4}"));
        assert_eq!(0, solve_puzzle("{\"a\":[-1,1]}"));
        assert_eq!(0, solve_puzzle("[-1, {\"a\":1}]"));
        assert_eq!(0, solve_puzzle("[]"));
        assert_eq!(0, solve_puzzle("{}"));
    }
}
