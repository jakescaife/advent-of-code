use std::collections::BTreeMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-04 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    input
        .lines()
        .map(parse_room)
        .filter(|x| room_code(x) == x.1)
        .fold(0, |s, x| s + x.2)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "aaaaa-bbb-z-y-x-123[abxyz]\n\
                    a-b-c-d-e-f-g-h-987[abcde]\n\
                    not-a-real-room-404[oarel]\n\
                    totally-real-room-200[decoy]";

        assert_eq!(1514, solve_puzzle(data));
    }
}
