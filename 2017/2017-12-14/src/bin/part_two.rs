use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-14 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let input = input.trim();

    let hashes: Vec<String> = (0..128)
        .map(|x| format!("{}-{}", input, x))
        .map(|x| binary_hash(&knot_hash(&x)))
        .collect();

    let graph: Vec<char> = hashes.iter()
        .flat_map(|x| x.chars())
        .collect();

    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    let mut regions = 0;

    for index in 0..graph.len() {
        let coordinates: (usize, usize) = (index % 128, index / 128);
        if !visited.insert(coordinates) || graph[index] == '0' {
            continue;
        }

        regions += 1;
        queue.push(coordinates);

        while let Some(coordinates) = queue.pop() {
            for next in fetch_adjacent(coordinates) {
                if visited.insert(next) && graph[next.0 + next.1 * 128] == '1' {
                    queue.push(next);
                }
            }
        }
    }

    regions
}

fn fetch_adjacent(coordinates: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    if coordinates.0 > 0 { adjacent.push((coordinates.0 - 1, coordinates.1)) }
    if coordinates.1 > 0 { adjacent.push((coordinates.0, coordinates.1 - 1)) }
    if coordinates.0 < 127 { adjacent.push((coordinates.0 + 1, coordinates.1)) }
    if coordinates.1 < 127 { adjacent.push((coordinates.0, coordinates.1 + 1)) }
    adjacent
}

fn binary_hash(hash: &str) -> String {
    hash.chars()
        .map(|x| x.to_digit(16).unwrap())
        .map(|x| format!("{:04b}", x))
        .collect()
}

fn knot_hash(input: &str) -> String {
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
    fn example_solutions() {
        assert_eq!(1242, solve_puzzle("flqrgnkx"));
    }
}
