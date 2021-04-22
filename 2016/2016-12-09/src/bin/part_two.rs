fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-09 Part Two: {}", solve_puzzle(&input));
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

    let compression_end = marker_end + characters + 1;

    let compressed_zone = &sequence[marker_end + 1..compression_end];
    let current_size = marker + repeats * decompress(compressed_zone);
    let sequence_tail = &sequence[compression_end..];

    current_size + decompress(sequence_tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(9, solve_puzzle("(3x3)XYZ"));
        assert_eq!(20, solve_puzzle("X(8x2)(3x3)ABCY"));
        assert_eq!(241920, solve_puzzle("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(
            445,
            solve_puzzle("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
