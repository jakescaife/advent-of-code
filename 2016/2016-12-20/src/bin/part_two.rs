fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-20 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let mut blacklist: Vec<_> = input.lines()
        .map(parse_range)
        .collect();

    blacklist.sort_unstable();

    let mut address = 0;
    let mut possible = 0;

    for range in blacklist {
        if address < range.0 {
            possible += range.0 - address;
        }

        let next_address = match range.1.checked_add(1) {
            Some(address) => address,
            None => break,
        };

        address = address.max(next_address);
    }

    possible
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
        assert_eq!(2, solve_puzzle("5-8\n0-2\n4-7\n10-10"));
    }
}
