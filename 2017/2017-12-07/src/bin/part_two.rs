use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-07 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> u32 {
    let mut nodes = HashMap::new();
    let mut names = HashSet::new();
    let mut above = HashSet::new();

    for mut entry in input.lines().map(|x| x.split_whitespace()) {
        let name = entry.next().unwrap();
        let mass = entry.next().unwrap()
            .trim_start_matches('(')
            .trim_end_matches(')');

        let mass = mass.parse().unwrap();

        names.insert(name);

        let tree: Vec<_> = entry.skip(1)
            .map(|x| x.trim_end_matches(','))
            .inspect(|&x| { above.insert(x); })
            .collect();

        nodes.insert(name, Node { mass, tree });
    }

    let root_node = names.difference(&above).next().unwrap();
    corrected_mass(root_node, &nodes)
}

struct Node<'a> {
    mass: u32,
    tree: Vec<&'a str>,
}

fn calculate_mass(node: &str, nodes: &HashMap<&str, Node>) -> u32 {
    let node = nodes.get(node).unwrap();
    let tree: u32 = node.tree.iter()
        .map(|x| calculate_mass(x, nodes))
        .sum();

    node.mass + tree
}

fn corrected_mass(node: &str, nodes: &HashMap<&str, Node>) -> u32 {
    let node = nodes.get(node).unwrap();

    let mut corrected_tree = node.tree.iter()
        .map(|x| corrected_mass(x, nodes));

    if let Some(x) = corrected_tree.find(|&x| x > 0) { return x; }

    let tree: Vec<_> = node.tree.iter()
        .map(|x| (x, calculate_mass(x, nodes)))
        .collect();

    if let Some(x) = tree.iter().find(|x| x.1 != tree[1].1) {
        let node = nodes.get(x.0).unwrap();
        return node.mass - (x.1 - tree[1].1);
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "pbga (66)\n\
                    xhth (57)\n\
                    ebii (61)\n\
                    havc (66)\n\
                    ktlj (57)\n\
                    fwft (72) -> ktlj, cntj, xhth\n\
                    qoyq (66)\n\
                    padx (45) -> pbga, havc, qoyq\n\
                    tknk (41) -> ugml, padx, fwft\n\
                    jptl (61)\n\
                    ugml (68) -> gyxo, ebii, jptl\n\
                    gyxo (61)\n\
                    cntj (57)";

        assert_eq!(60, solve_puzzle(data));
    }
}
