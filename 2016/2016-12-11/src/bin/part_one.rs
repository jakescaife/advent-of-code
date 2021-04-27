use std::collections::HashSet;
use std::collections::VecDeque;
use std::iter;
use std::collections::BTreeSet;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("AOC 2016-11 Part One: {}", solve_puzzle(&input));
}

fn solve_puzzle(input: &str) -> usize {
    let initial_state = State::initial_state(input);

    println!("INITIAL LEGAL: {}", initial_state.valid_state());

    let mut queue: VecDeque<_> = std::iter::once(initial_state).collect();
    let mut duplicates: HashSet<_> = queue.iter().cloned().collect();

    while let Some(state) = queue.pop_front() {

        println!("DEPTH: {}, DUPLICATES: {}", state.moves, duplicates.len());

        if state.target_state() {
            return state.moves;
        }

        for next_state in state.possible_states() {
            if !duplicates.insert(next_state.clone()) {
                continue;
            }

            queue.push_back(next_state);
        }
    }



    unreachable!()
}


fn object_sets(floor: &BTreeSet<Object>) -> (HashSet<&String>, HashSet<&String>) {
    let mut microchips = HashSet::new();
    let mut generators = HashSet::new();

    for object in floor.iter() {
        match object {
            Object::Generator(x) => generators.insert(x),
            Object::Microchip(x) => microchips.insert(x),
            Object::Combinations(_) => continue,
        };
    }

    (microchips, generators)
}

fn legal_set(floor: &BTreeSet<Object>) -> bool {
    let (microchips, generators) = object_sets(floor);
    generators.is_empty() || microchips.is_subset(&generators)
}

#[derive(Eq, Clone)]
struct State {
    moves: usize,
    elevator: usize,
    floors: Vec<BTreeSet<Object>>,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct ComparableState {
    elevator: usize,
    floors: Vec<BTreeSet<Object>>,
}

impl State {
    fn initial_state(description: &str) -> Self {
        let floors = description.lines().map(|x| {
            x.split(" a ").skip(1)
                .map(|x| x.trim_end_matches(" and"))
                .map(|x| x.trim_end_matches(|x| matches!(x, ',' | '.')))
                .map(|x| Object::new(x))
                .collect()
        });

        Self { moves: 0, elevator: 0, floors: floors.collect() }
    }

    fn valid_state(&self) -> bool {
        self.floors.iter().all(|x| legal_set(x))
    }

    fn comparable(&self) -> ComparableState {
        let floors = self.floors.iter().map(|floor| {
            let (microchips, generators) = object_sets(floor);

            let matching = microchips.intersection(&generators).count();
            let matching = iter::once(Object::Combinations(matching));

            let lone_generators = generators.difference(&microchips)
                .map(|x| Object::Generator(x.to_string()));

            let lone_microchips = microchips.difference(&generators)
                .map(|x| Object::Microchip(x.to_string()));

            matching.chain(lone_generators).chain(lone_microchips).collect()

        });

        ComparableState { elevator: self.elevator, floors: floors.collect() }
    }

    fn target_state(&self) -> bool {
        self.floors.iter().take(3).all(|x| x.is_empty())
    }

    fn next_state(&self, objects: &[&Object], destination: usize) -> Self {
        let mut floors = self.floors.clone();

        for object in objects {
            let object = floors[self.elevator].take(object).unwrap();
            floors[destination].insert(object);
        }

        Self {
            moves: self.moves + 1,
            elevator: destination,
            floors
        }
    }

    fn possible_states(self) -> HashSet<Self> {
        let mut states = HashSet::new();

        let destinations: &[usize] = match self.elevator {
            0 => &[1],
            1 => &[0, 2],
            2 => &[1, 3],
            _ => &[2],
        };

        for destination in destinations {
            let floor = self.floors.get(self.elevator).unwrap();
            for item in floor.iter() {
                states.insert(self.next_state(&[item], *destination));
                for pair in floor.iter() {
                    if item == pair {
                        continue;
                    }

                    states.insert(self.next_state(&[item, pair], *destination));
                }
            }
        }

        let valid: HashSet<Self> = states.into_iter().filter(|x| x.valid_state()).collect();

        valid
    }
}


impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.comparable() == other.comparable()
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.comparable().hash(state);
    }
}


impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "-------------STATE----------")?;
        writeln!(f, "MOVES: {}, ELEVATOR: F{}", self.moves, self.elevator)?;
        for (num, floor) in self.floors.iter().enumerate().rev() {
            write!(f, "F{}: ", num + 1)?;
            for item in floor.iter() {
                write!(f, "{:?} ", item)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[derive(Debug)]
enum Object {
    Generator(String),
    Microchip(String),
    Combinations(usize),
}

impl Object {
    fn new(description: &str) -> Self {
        let mut description = description.split(|x| x == ' ' || x == '-');
        let material = description.next().unwrap().to_string();
        match description.last() {
            Some("generator") => Self::Generator(material),
            Some("microchip") => Self::Microchip(material),
            _ => panic!("Unexpected Object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_solutions() {
        let data = "The first floor contains a hydrogen-compatible microchip and a \
            lithium-compatible microchip.\n\
                    The second floor contains a hydrogen generator.\n\
                    The third floor contains a lithium generator.\n\
                    The fourth floor contains nothing relevant.";

        assert_eq!(11, solve_puzzle(data));
    }
}
