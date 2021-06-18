use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-12 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let connections: Vec<Vec<_>> = input.lines()
        .map(fetch_connections)
        .collect();

    let mut programs: HashSet<usize> = (0..input.lines().count()).collect();
    let mut groups = 0;

    while let Some(program) = programs.iter().next() {
        let collection = group(*program, &connections);
        programs = programs.difference(&collection).copied().collect();
        groups += 1;
    }

    groups
}

fn group(program: usize, connections: &Vec<Vec<usize>>) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut queue = vec![program];

    while let Some(program) = queue.pop() {
        if visited.insert(program) {
            queue.extend_from_slice(&connections[program])
        }
    }

    visited
}

fn fetch_connections(description: &str) -> Vec<usize> {
    let mut connections = description.split(" <-> ").skip(1);
    connections.next().unwrap().split(", ")
        .filter_map(|x| x.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let example = "0 <-> 2\n1 <-> 1\n2 <-> 0, 3, 4\n3 <-> 2, 4\n\
                       4 <-> 2, 3, 6\n5 <-> 6\n6 <-> 4, 5";
        assert_eq!(2, solve_puzzle(example));
    }
}
