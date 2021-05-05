fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-18 Part Two: {}", solve_puzzle(&input, 400000));
}

fn solve_puzzle(input: &str, rows: usize) -> usize {
    let mut current = input.trim().to_owned();
    let mut count = safe_tiles(&current);

    for _ in 1..rows {
        current = next_line(&current);
        count += safe_tiles(&current);
    }

    count
}

fn safe_tiles(row: &str) -> usize {
    row.chars().filter(|&x| x == '.').count()
}

fn next_line(current: &str) -> String {
    let current = current.as_bytes();
    (0..current.len()).map(|x| next_tile(current, x)).collect()
}

fn next_tile(current: &[u8], index: usize) -> char {
    let trapped_west = index.checked_sub(1)
        .map(|x| current[x] == b'^')
        .unwrap_or(false);

    let trapped_east = current.get(index + 1)
        .map(|&x| x == b'^')
        .unwrap_or(false);

    let only_west = trapped_west && !trapped_east;
    let only_east = trapped_east && !trapped_west;

    if only_west || only_east { '^' } else { '.' }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(38, solve_puzzle(".^^.^.^^^^", 10));
    }
}
