fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-10 Part One: {}", solve_puzzle(&input, 256));
}

fn solve_puzzle(input: &str, size: usize) -> usize {
    let lengths: Vec<usize> = input.trim().split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut rope: Vec<usize> = (0..size).collect();
    let mut position = 0;
    let mut skip = 0;

    for length in lengths {
        reverse(&mut rope, position, length);
        position = (position + length + skip) % size;
        skip += 1;
    }

    rope[0] * rope[1]
}

fn reverse(section: &mut [usize], position: usize, length: usize) {
    section.rotate_left(position);
    section[0..length].reverse();
    section.rotate_right(position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        assert_eq!(12, solve_puzzle("3,4,1,5", 5));
    }
}
