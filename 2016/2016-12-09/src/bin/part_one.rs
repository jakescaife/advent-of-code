fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-09 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let input: String = input.lines().collect();
    decompress(&input)
}

fn decompress(sequence: &str) -> usize {
    let marker = match sequence.find('(') {
        Some(x) => x,
        None => return sequence.len(),
    };

    let marker_end = sequence.find(')').unwrap();
    let mut dimensions = sequence[marker + 1..marker_end]
        .split('x').map(|x| x.parse().unwrap());

    let characters: usize = dimensions.next().unwrap();
    let repeats: usize = dimensions.next().unwrap();

    let current_size = marker + characters * repeats;
    let sequence_tail = &sequence[marker_end + characters + 1..];

    current_size + decompress(sequence_tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(6, solve_puzzle("ADVENT"));
        assert_eq!(7, solve_puzzle("A(1x5)BC"));
        assert_eq!(9, solve_puzzle("(3x3)XYZ"));
        assert_eq!(6, solve_puzzle("(6x1)(1x3)A"));
        assert_eq!(11, solve_puzzle("A(2x2)BCD(2x2)EFG"));
        assert_eq!(18, solve_puzzle("X(8x2)(3x3)ABCY"));
    }
}
