fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-22 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let nodes: Vec<Node> = input.lines().skip(2).map(parse_node).collect();

    let max_x = nodes.iter().fold(0, |s, x| s.max(x.x)) + 1;
    let max_y = nodes.iter().fold(0, |s, x| s.max(x.y)) + 1;

    let empty = nodes.iter().find(|x| x.used == 0).unwrap();
    let mut chart = vec!['_'; max_x * max_y];

    for node in nodes.iter() {
        let character = if node.used <= empty.available { '.' } else { '#' };
        chart[node.y * max_x + node.x] = character
    }

    chart[max_x - 1] = 'G';
    chart[empty.y * max_x + empty.x] = 'E';

    let mut display = String::new();

    for (index, node) in chart.iter().enumerate() {
        if index % max_x == 0 { display.push('\n'); }
        display.push(*node);
    }

    display
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
