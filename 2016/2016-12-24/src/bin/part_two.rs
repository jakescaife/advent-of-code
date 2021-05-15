use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-24 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let maze: Vec<char> = input.lines().flat_map(|x| x.chars()).collect();
    let maze_width = input.find(|x: char| x.is_whitespace()).unwrap();

    let mut targets: Vec<_> = maze.iter()
        .enumerate()
        .filter(|x| x.1.is_numeric())
        .collect();

    let start_point = targets.iter().position(|x| x.1 == &'0').unwrap();
    let start_point = targets.remove(start_point);
    targets.insert(0, start_point);

    let mut targets: Vec<_> = targets.into_iter()
        .map(|x| x.0)
        .collect();

    let mut cost_map = HashMap::new();

    for a in 0..targets.len() {
        for b in a + 1..targets.len() {
            let a = targets[a];
            let b = targets[b];
            let cost = cost(&maze, maze_width, a, b);
            cost_map.insert((a, b), cost);
            cost_map.insert((b, a), cost);
        }
    }

    let mut min_distance = path_distance(&targets, &cost_map);

    while next_permutation(&mut targets[1..]).is_some() {
        let distance = path_distance(&targets, &cost_map);
        min_distance = min_distance.min(distance);
    }

    min_distance
}

type CostMap = HashMap<(usize, usize), usize>;

fn path_distance(path: &[usize], costs: &CostMap) -> usize {
    let distance: usize = path.windows(2)
        .map(|x| (x[0], x[1]))
        .map(|x| costs[&x])
        .sum();

    let return_start = (path[path.len() - 1], path[0]);
    distance + costs[&return_start]
}

fn cost(maze: &[char], width: usize, start: usize, end: usize) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((position, cost)) = queue.pop_front() {
        let adjacent = &[
            position + 1,
            position - 1,
            position + width,
            position - width,
        ];

        for position in adjacent {
            if position == &end { return cost + 1 }
            if maze[*position] != '#' && visited.insert(*position) {
                queue.push_back((*position, cost + 1));
            }
        }
    }

    panic!("Unable to find route from {} to {}", start, end)
}

fn next_permutation(positions: &mut [usize]) -> Option<()> {
    let pivot = positions.windows(2).rev()
        .position(|x| x[0] < x[1])
        .map(|x| positions.len() - x - 2)?;

    let successor = positions.iter().rev()
        .position(|&x| x > positions[pivot])
        .map(|x| positions.len() - x - 1)?;

    positions.swap(pivot, successor);
    positions[pivot + 1..].reverse();
    Some(())
}
