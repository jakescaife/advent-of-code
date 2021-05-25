use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-07 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> &str {
    let mut names = HashSet::new();
    let mut above = HashSet::new();

    for mut entry in input.lines().map(|x| x.split_whitespace()) {
        names.insert(entry.next().unwrap());
        entry.skip(2)
            .map(|x| x.trim_end_matches(','))
            .for_each(|x| { above.insert(x); });
    }

    names.difference(&above).next().unwrap()
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

        assert_eq!("tknk", solve_puzzle(data));
    }
}
