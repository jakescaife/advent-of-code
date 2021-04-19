use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-06 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> String {
    let words: Vec<_> = input.lines().map(|x| x.as_bytes()).collect();
    let mut message = String::new();

    for column in 0..words[0].len() {
        let mut count = HashMap::new();

        for character in words.iter().map(|x| x[column] as char) {
            *count.entry(character).or_insert(0) += 1;
        }

        let most_common = count.into_iter().max_by_key(|x| x.1).map(|x| x.0);
        message.push(most_common.unwrap());
    }

    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\n\
                    nssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar";

        assert_eq!("easter", &solve_puzzle(data));
    }
}
