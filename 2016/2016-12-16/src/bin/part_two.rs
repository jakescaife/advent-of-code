fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-16 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let data = generate_data(input.trim().into(), 35651584);
    generate_checksum(data)
}

fn generate_data(mut data: String, length: usize) -> String {
    if data.len() >= length {
        data.truncate(length);
        return data;
    }

    let tail: String = data.chars().rev()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect();

    let data = format!("{}0{}", data, tail);
    generate_data(data, length)
}

fn generate_checksum(data: String) -> String {
    if data.len() % 2 == 1 { return data; }

    let checksum: String = data.as_bytes().chunks(2)
        .map(|x| if x[0] == x[1] { '1' } else { '0' })
        .collect();

    generate_checksum(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = generate_data("10000".into(), 20);
        assert_eq!("01100", &generate_checksum(data));
    }
}
