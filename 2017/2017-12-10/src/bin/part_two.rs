fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-10 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let mut lengths: Vec<u8> = input.trim().bytes().collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);

    let mut rope: Vec<u8> = (0..=255).collect();
    let mut position = 0;
    let mut skip = 0;

    for _ in 0..64 {
        for &length in lengths.iter() {
            reverse(&mut rope, position, length as usize);
            position = (position + skip + length as usize) % 256;
            skip += 1;
        }
    }

    rope.chunks(16)
        .map(|x| x.iter().fold(0, |x, s| x ^ s))
        .map(|x| format!("{:02x}", x))
        .collect()
}

fn reverse(section: &mut [u8], position: usize, length: usize) {
    section.rotate_left(position);
    section[0..length].reverse();
    section.rotate_right(position);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", &solve_puzzle(""));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", &solve_puzzle("AoC 2017"));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", &solve_puzzle("1,2,3"));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", &solve_puzzle("1,2,4"));
    }
}
