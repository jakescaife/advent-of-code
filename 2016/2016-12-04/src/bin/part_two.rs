use std::collections::BTreeMap;
use std::convert::TryFrom;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-04 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    input
        .lines()
        .map(parse_room)
        .filter(|x| room_code(x) == x.1)
        .find(|x| plaintext(&x.0, x.2).contains("north"))
        .map(|x| x.2)
        .unwrap()
}

type Room = (String, String, u32);

fn parse_room(room: &str) -> Room {
    let room = room.replace("[", "-");
    let room = room.replace("]", "");
    let mut room = room.rsplit('-');

    let checksum = room.next().unwrap().to_string();
    let id = room.next().unwrap().parse().unwrap();
    let name: String = room.collect();

    (name, checksum, id)
}

fn room_code(room: &Room) -> String {
    let mut count = BTreeMap::new();
    for c in room.0.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    let mut results: Vec<_> = count.iter().collect();
    results.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    results.iter().take(5).map(|x| x.0).collect()
}

fn plaintext(encrypted: &str, id: u32) -> String {
    encrypted.chars().map(|x| shift_character(x, id)).collect()
}

fn shift_character(c: char, x: u32) -> char {
    let c = c as u32 - 'a' as u32;
    char::try_from((c + x) % 26 + 'a' as u32).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        assert_eq!(
            "veryencryptedname",
            &plaintext("qzmtzixmtkozyivhz", 343)
        );
    }
}
