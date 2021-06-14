fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2017-11 Part Two: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> i32 {
    let positions = input.trim().split(',').scan((0, 0, 0), |s, x| {
        *s = next_hex(*s, x);
        Some(*s)
    });

    positions.map(|x| (x.0.abs() + x.1.abs() + x.2.abs()) / 2)
        .max().unwrap()
}

fn next_hex(mut position: (i32, i32, i32), direction: &str) -> (i32, i32, i32) {
    match direction {
        "n"  => { position.1 += 1; position.2 -= 1 },
        "s"  => { position.1 -= 1; position.2 += 1 },
        "ne" => { position.0 += 1; position.2 -= 1 },
        "sw" => { position.0 -= 1; position.2 += 1 },
        "nw" => { position.0 -= 1; position.1 += 1 },
        "se" => { position.0 += 1; position.1 -= 1 },
        other => panic!("Unexpected direction: {}", other),
    }

    position
}
