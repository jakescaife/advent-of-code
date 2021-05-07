fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-20 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let mut blacklist: Vec<_> = input.lines()
        .map(parse_range)
        .collect();

    blacklist.sort_unstable();

    let mut address = 0;

    for range in blacklist {
        if address < range.0 {
            break;
        }

        address = address.max(range.1 + 1);
    }

    address
}

fn parse_range(description: &str) -> (u32, u32) {
    let description: Vec<u32> = description.split('-')
        .map(|x| x.parse().unwrap())
        .collect();
    (description[0], description[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(3, solve_puzzle("5-8\n0-2\n4-7"));
    }
}
