fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-22 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let nodes: Vec<Node> = input.lines().skip(2).map(parse_node).collect();

    nodes.iter()
        .filter(|x| x.used > 0)
        .map(|x| viable_pairs(x, &nodes))
        .sum()
}

struct Node { x: usize, y: usize, used: usize, available: usize }

fn parse_node(node: &str) -> Node {
    let node: Vec<usize> = node
        .split(|c: char| c.is_whitespace() || c == '-')
        .map(|x| x.trim_start_matches(|c| c == 'x' || c == 'y'))
        .map(|x| x.trim_end_matches(|c| c == 'T'))
        .filter_map(|x| x.parse().ok())
        .collect();

    Node { x: node[0], y: node[1], used: node[3], available: node[4] }
}

fn viable_pairs(x: &Node, nodes: &[Node]) -> usize {
    nodes.iter().filter(|y| x.used <= y.available && !(x.x == y.x && x.y == y.y)).count()
}
