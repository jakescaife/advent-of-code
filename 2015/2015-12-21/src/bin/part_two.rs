fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-21 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i32 {
    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

    let clothes = [
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];

    let rings = [
        (0, 0, 0),
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];

    let boss = Player::from(input);
    let player = Player {
        health: 100,
        damage: 0,
        resist: 0,
        cost: 0,
    };

    let mut maximum_cost = i32::MIN;

    for weapon in &weapons {
        for armor in &clothes {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1 == ring2 && ring1 != &(0, 0, 0) {
                        continue;
                    }

                    let player = player.with_items(weapon, armor, ring1, ring2);
                    maximum_cost = maximum_cost.max(player.fight_cost(&boss));
                }
            }
        }
    }

    maximum_cost
}

#[derive(Copy, Clone)]
struct Player {
    health: i32,
    damage: i32,
    resist: i32,
    cost: i32,
}

type Item = (i32, i32, i32);

impl Player {
    fn with_items(&self, weapon: &Item, armor: &Item, ring1: &Item, ring2: &Item) -> Self {
        Self {
            health: self.health,
            damage: weapon.1 + ring1.1 + ring2.1,
            resist: armor.2 + ring1.2 + ring2.2,
            cost: weapon.0 + armor.0 + ring1.0 + ring2.0,
        }
    }

    fn fight_cost(&self, opponent: &Self) -> i32 {
        let damage = (self.damage - opponent.resist).max(1);
        let required_hits = (opponent.health + damage - 1) / damage;
        let damage = (opponent.damage - self.resist).max(1);

        if required_hits > (self.health + damage - 1) / damage {
            self.cost
        } else {
            i32::MIN
        }
    }
}

impl From<&str> for Player {
    fn from(s: &str) -> Self {
        let sentence: Vec<i32> = s
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        Self {
            health: sentence[0],
            damage: sentence[1],
            resist: sentence[2],
            cost: 0,
        }
    }
}
