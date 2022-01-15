use std::collections::HashMap;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day21.txt";

// A throw of 3 3-sided dice are guaranteed to give a score between 3 and 9.
// This table records how many of all 27 (3^3) possible throws give a given score.
const SCORES: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    pub fn new(pos: u64, score: u64) -> Self {
        Self { pos, score }
    }

    pub fn advance(&self, value: u64) -> Self {
        let pos = (self.pos + value - 1) % 10 + 1;
        let score = self.score + pos;
        Self::new(pos, score)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    p1: Player,
    p2: Player,
}

impl State {
    pub fn new(p1: Player, p2: Player) -> Self {
        Self { p1, p2 }
    }
}

fn part1(p1: &Player, p2: &Player) -> Result<(), Error> {
    let mut roll_count = 0;

    let mut p1 = p1.clone();
    let mut p2 = p2.clone();

    let answer = loop {
        let roll = (0..3).fold(0, |mut roll, _| {
            roll += roll_count % 100 + 1;
            roll_count += 1;
            roll
        });

        p1 = p1.advance(roll);
        if p1.score >= 1000 {
            break p2.score * roll_count;
        }

        let roll = (0..3).fold(0, |mut roll, _| {
            roll += roll_count % 100 + 1;
            roll_count += 1;
            roll
        });

        p2 = p2.advance(roll);
        if p2.score >= 1000 {
            break p1.score * roll_count;
        }
    };

    println!("part1: {answer}");
    Ok(())
}

fn part2(p1: &Player, p2: &Player) -> Result<(), Error> {
    let state = State::new(p1.clone(), p2.clone());
    let mut cache = HashMap::new();

    let (p1_wins, p2_wins) = dirac_dice(&mut cache, state);

    let answer = p1_wins.max(p2_wins);

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;
    let mut lines = input.trim().lines();

    let p1 = lines.next().ok_or_else(|| Error::msg("invalid input"))?;
    let p2 = lines.next().ok_or_else(|| Error::msg("invalid input"))?;

    let p1 = p1
        .trim()
        .split_whitespace()
        .nth(4)
        .ok_or_else(|| Error::msg("invalid input"))?;
    let p2 = p2
        .trim()
        .split_whitespace()
        .nth(4)
        .ok_or_else(|| Error::msg("invalid input"))?;

    let p1 = p1.parse()?;
    let p2 = p2.parse()?;

    let p1 = Player { pos: p1, score: 0 };
    let p2 = Player { pos: p2, score: 0 };

    measured!(part1(&p1, &p2))?;
    measured!(part2(&p1, &p2))?;

    Ok(())
}

// This solution is basically a simple divide-and-conquer (recursive) solution.
// But we add memoization to drastically accelerate it since similar states are
// very likely to appear across multiple universes.
// PS: we take turns into account by simply inverting the players before our recursive call.
fn dirac_dice(cache: &mut HashMap<State, (u64, u64)>, state: State) -> (u64, u64) {
    if let Some(result) = cache.get(&state).copied() {
        return result;
    }

    let mut wins = (0, 0);

    for (score, amount) in SCORES {
        let new_p1 = state.p1.advance(score);

        if new_p1.score >= 21 {
            wins.0 += amount;
        } else {
            let state = State::new(state.p2.clone(), new_p1);
            let (p2_wins, p1_wins) = dirac_dice(cache, state);
            wins.0 += p1_wins * amount;
            wins.1 += p2_wins * amount;
        }
    }

    cache.insert(state, wins);
    wins
}
