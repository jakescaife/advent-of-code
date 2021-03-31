fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-18 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> usize {
    let mut lights = LightGrid::from(input);

    for _ in 0..100 {
        lights.animate();
    }

    lights.count()
}

struct LightGrid {
    status: Vec<bool>,
    size: usize,
}

impl LightGrid {
    fn animate(&mut self) {
        let mut next_status = self.status.clone();

        for y in 1..self.size - 1 {
            for x in 1..self.size - 1 {
                let index = x + (y * self.size);
                next_status[index] = self.calculate_state(index);
            }
        }

        next_status[self.size + 1] = true;
        next_status[self.size * 2 - 2] = true;
        next_status[self.size * (self.size - 2) + 1] = true;
        next_status[self.size * (self.size - 1) - 2] = true;

        self.status = next_status;
    }

    fn calculate_state(&self, index: usize) -> bool {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let neighbours = directions
            .iter()
            .map(|(x, y)| (self.size as i32 * y) + x + index as i32)
            .filter(|x| self.status[*x as usize])
            .count();

        neighbours == 3 || neighbours == 2 && self.status[index]
    }

    fn count(&self) -> usize {
        self.status.iter().filter(|x| **x).count()
    }
}

impl From<&str> for LightGrid {
    fn from(s: &str) -> Self {
        let mut unbordered: Vec<Vec<bool>> = s
            .lines()
            .map(|x| x.chars().map(|x| x == '#').collect())
            .collect();

        let size = s.lines().count();
        unbordered.insert(0, vec![false; size]);
        unbordered.push(vec![false; size]);

        for x in unbordered.iter_mut() {
            x.insert(0, false);
            x.push(false);
        }

        let status = unbordered.concat();
        let size = size + 2;

        Self { status, size }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let mut lights = LightGrid::from("##.#.#\n...##.\n#....#\n..#...\n#.#..#\n####.#");

        for _ in 0..5 {
            lights.animate();
        }

        assert_eq!(17, lights.count());
    }
}
