fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-15 Part One: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i32 {
    let ingredients: Vec<_> = input.lines().map(Ingredient::from).collect();
    let mut optimal = i32::MIN;

    for x in 0..=100 {
        for y in 0..=100 - x {
            for z in 0..=100 - x - y {
                let combination = ingredients[0] * x
                    + ingredients[1] * y
                    + ingredients[2] * z
                    + ingredients[3] * (100 - x - y - z);
                optimal = optimal.max(combination.score())
            }
        }
    }

    optimal
}

#[derive(Copy, Clone)]
struct Ingredient(i32, i32, i32, i32, i32);

impl Ingredient {
    fn score(&self) -> i32 {
        self.0 * self.1 * self.2 * self.3
    }
}

impl std::ops::Mul<i32> for Ingredient {
    type Output = Self;
    fn mul(self, x: i32) -> Self {
        Self(self.0 * x, self.1 * x, self.2 * x, self.3 * x, self.4 * x)
    }
}

impl std::ops::Add for Ingredient {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(
            (self.0 + other.0).max(0),
            (self.1 + other.1).max(0),
            (self.2 + other.2).max(0),
            (self.3 + other.3).max(0),
            (self.4 + other.4).max(0),
        )
    }
}

impl From<&str> for Ingredient {
    fn from(s: &str) -> Self {
        let sentence: Vec<i32> = s
            .split_whitespace()
            .filter_map(|x| x.trim_end_matches(',').parse().ok())
            .collect();

        Self(
            sentence[0],
            sentence[1],
            sentence[2],
            sentence[3],
            sentence[4],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                    Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let ingredients: Vec<_> = data.lines().map(Ingredient::from).collect();
        let mut optimal = i32::MIN;

        for x in 0..=100 {
            let combination = ingredients[0] * x + ingredients[1] * (100 - x);
            optimal = optimal.max(combination.score());
        }

        assert_eq!(62842880, optimal);
    }
}
