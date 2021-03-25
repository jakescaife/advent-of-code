fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-14 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    let mut raindeer: Vec<_> = input.lines().map(Raindeer::from).collect();

    for _ in 0..2503 {
        let leading = raindeer.iter_mut().fold(0, |max, x| {
            x.race();
            max.max(x.total_distance)
        });

        raindeer
            .iter_mut()
            .filter(|x| x.total_distance == leading)
            .for_each(|x| x.points += 1);
    }

    raindeer.iter().fold(0, |max, x| max.max(x.points))
}

struct Raindeer {
    speed: u32,
    flight: u32,
    rest: u32,
    current: u32,
    total_distance: u32,
    state: State,
    points: u32,
}

enum State {
    Flying,
    Resting,
}

impl Raindeer {
    fn race(&mut self) {
        match self.state {
            State::Flying => self.fly(),
            State::Resting => self.rest(),
        }
    }

    fn fly(&mut self) {
        self.total_distance += self.speed;
        self.current += 1;

        if self.current == self.flight {
            self.current = 0;
            self.state = State::Resting;
        }
    }

    fn rest(&mut self) {
        self.current += 1;

        if self.current == self.rest {
            self.current = 0;
            self.state = State::Flying;
        }
    }
}

impl From<&str> for Raindeer {
    fn from(s: &str) -> Self {
        let sentence: Vec<&str> = s.split_whitespace().collect();
        let speed = sentence[3].parse().unwrap();
        let flight = sentence[6].parse().unwrap();
        let rest = sentence[13].parse().unwrap();

        Self {
            speed,
            flight,
            rest,
            current: 0,
            total_distance: 0,
            state: State::Flying,
            points: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
                    Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let mut raindeer: Vec<_> = data.lines().map(Raindeer::from).collect();

        for _ in 0..1000 {
            raindeer.iter_mut().for_each(|x| x.race());
        }

        assert_eq!(
            1120,
            raindeer.iter().fold(0, |max, x| max.max(x.total_distance))
        );
    }
}
