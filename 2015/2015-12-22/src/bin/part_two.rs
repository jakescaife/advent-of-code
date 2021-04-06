fn main() {
    let puzzle_solution = std::fs::read_to_string("input.txt")
        .map(|input| solve_puzzle(&input))
        .expect("Error while reading puzzle input file.");

    println!("AOC 2015-22 Part Two: {}", puzzle_solution);
}

fn solve_puzzle(input: &str) -> i32 {
    let boss = Boss::from(input);
    let fighter = Fighter::new(50, 500);

    minimum_mana_spend(fighter, boss, i32::MAX)
}

fn minimum_mana_spend(mut fighter: Fighter, mut boss: Boss, mut minimum_spend: i32) -> i32 {
    let spells = [
        Spell { cost:  53, damage: 4, heal: 0, resist: 0, mana:   0, duration: 0 },
        Spell { cost:  73, damage: 2, heal: 2, resist: 0, mana:   0, duration: 0 },
        Spell { cost: 113, damage: 0, heal: 0, resist: 7, mana:   0, duration: 6 },
        Spell { cost: 173, damage: 3, heal: 0, resist: 0, mana:   0, duration: 6 },
        Spell { cost: 229, damage: 0, heal: 0, resist: 0, mana: 101, duration: 5 },
    ];

    fighter.health -= 1;
    fighter.apply_effects(&mut boss);

    if boss.is_dead() {
        return fighter.spent;
    } else if fighter.is_dead() {
        return minimum_spend;
    }

    let available_spells = spells
        .iter()
        .filter(|x| x.cost <= fighter.mana)
        .filter(|x| !fighter.effects.contains(x));

    for spell in available_spells {
        if fighter.spent + spell.cost >= minimum_spend {
            continue;
        }

        let mut boss = boss;
        let mut fighter = fighter.with_spell(*spell);

        fighter.apply_effects(&mut boss);
        fighter.health -= (boss.damage - fighter.resist).max(1);

        let spent_mana = if boss.is_dead() {
            fighter.spent
        } else if fighter.is_dead() {
            continue;
        } else {
            minimum_mana_spend(fighter, boss, minimum_spend)
        };

        minimum_spend = minimum_spend.min(spent_mana);
    }

    minimum_spend
}

#[derive(Clone)]
struct Fighter {
    health: i32,
    mana: i32,
    resist: i32,
    spent: i32,
    effects: Vec<Spell>,
}

impl Fighter {
    fn new(health: i32, mana: i32) -> Self {
        Self {
            health,
            mana,
            resist: 0,
            spent: 0,
            effects: vec![],
        }
    }

    fn with_spell(&self, spell: Spell) -> Self {
        let mut fighter = self.clone();
        fighter.resist = 0;
        fighter.mana -= spell.cost;
        fighter.spent += spell.cost;
        fighter.effects.push(spell);
        fighter
    }

    fn apply_effects(&mut self, boss: &mut Boss) {
        for effect in self.effects.iter_mut() {
            boss.health -= effect.damage;
            self.health += effect.heal;
            self.resist += effect.resist;
            self.mana += effect.mana;
            effect.duration -= 1;
        }

        self.effects = self.effects.drain(..).filter(|x| x.duration > 0).collect();
    }

    fn is_dead(&self) -> bool {
        self.health <= 0
    }
}

#[derive(Copy, Clone)]
struct Spell {
    cost: i32,
    damage: i32,
    heal: i32,
    resist: i32,
    mana: i32,
    duration: i32,
}

impl PartialEq for Spell {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Copy, Clone)]
struct Boss {
    health: i32,
    damage: i32,
}

impl Boss {
    fn is_dead(&self) -> bool {
        self.health <= 0
    }
}

impl From<&str> for Boss {
    fn from(s: &str) -> Self {
        let sentence: Vec<_> = s
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        Self {
            health: sentence[0],
            damage: sentence[1],
        }
    }
}
