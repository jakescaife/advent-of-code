fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-06 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> u32 {
    input
        .lines()
        .fold(LightGrid::default(), |mut state, task| {
            state.execute_task(task.into());
            state
        })
        .count()
}

struct LightGrid {
    status: Vec<u32>,
}

struct Task {
    action: Action,
    start: Position,
    end: Position,
}

#[derive(Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle,
}

struct Position {
    x: usize,
    y: usize,
}

impl LightGrid {
    fn set_light(&mut self, x: usize, y: usize, action: Action) {
        let light = &mut self.status[x + y * 1000];
        *light = match action {
            Action::On => *light + 1,
            Action::Off => light.checked_sub(1).unwrap_or(0),
            Action::Toggle => *light + 2,
        };
    }

    fn execute_task(&mut self, task: Task) {
        for x in task.start.x..=task.end.x {
            for y in task.start.y..=task.end.y {
                self.set_light(x, y, task.action);
            }
        }
    }

    fn count(&self) -> u32 {
        self.status.iter().sum()
    }
}

impl Default for LightGrid {
    fn default() -> Self {
        Self {
            status: vec![0; 1000 * 1000],
        }
    }
}

impl From<&str> for Position {
    fn from(s: &str) -> Self {
        let coords: Vec<usize> = s.split(',').map(|x| x.parse().unwrap()).collect();
        let x = coords[0];
        let y = coords[1];

        Self { x, y }
    }
}

impl From<&str> for Task {
    fn from(s: &str) -> Self {
        let s = s.replace("turn", "");
        let s = s.replace("through", "");
        let s: Vec<&str> = s.split_whitespace().collect();

        let action = match s[0] {
            "on" => Action::On,
            "off" => Action::Off,
            "toggle" => Action::Toggle,
            _ => panic!("Unexpected action."),
        };

        let start = Position::from(s[1]);
        let end = Position::from(s[2]);

        Self { action, start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solution() {
        let instructions = "turn on 0,0 through 0,0\n\
                            toggle 0,0 through 999,999";

        assert_eq!(2000001, solve_puzzle(instructions));
    }
}
