use std::collections::HashMap;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day14.txt";

type FrequencyMap = HashMap<(char, char), usize>;
type InsertionMap = HashMap<(char, char), char>;

fn part1(initial: &FrequencyMap, map: &InsertionMap) -> Result<(), Error> {
    let mut fst = initial.clone();
    let mut snd = HashMap::with_capacity(fst.capacity());

    (0..10).for_each(|_| {
        tick(map, &fst, &mut snd);
        std::mem::swap(&mut fst, &mut snd);
        snd.clear();
    });

    let answer = compute_score(&fst);

    println!("part1: {}", answer);
    Ok(())
}

fn part2(initial: &FrequencyMap, map: &InsertionMap) -> Result<(), Error> {
    let mut fst = initial.clone();
    let mut snd = HashMap::with_capacity(fst.capacity());

    (0..40).for_each(|_| {
        tick(map, &fst, &mut snd);
        std::mem::swap(&mut fst, &mut snd);
        snd.clear();
    });

    let answer = compute_score(&fst);

    println!("part2: {}", answer);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let (initial, map) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| Error::msg("invalid input"))?;

    let initial: FrequencyMap = {
        let mut map = HashMap::new();
        let chars: Vec<char> = initial.chars().collect();
        for w in chars.windows(2) {
            *map.entry((w[0], w[1])).or_default() += 1;
        }
        map
    };

    let map: InsertionMap = map
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ")?;

            let (a, b) = from.chars().nth(0).zip(from.chars().nth(1))?;
            let c = to.chars().nth(0)?;

            Some(((a, b), c))
        })
        .collect::<Option<_>>()
        .ok_or_else(|| Error::msg("invalid input"))?;

    measured!(part1(&initial, &map))?;
    measured!(part2(&initial, &map))?;

    Ok(())
}

fn tick(map: &InsertionMap, fst: &FrequencyMap, snd: &mut FrequencyMap) {
    for (&(ch1, ch2), &freq) in fst {
        if let Some(ch) = map.get(&(ch1, ch2)).copied() {
            *snd.entry((ch1, ch)).or_default() += freq;
            *snd.entry((ch, ch2)).or_default() += freq;
        }
    }
}

fn compute_score(freq_map: &FrequencyMap) -> usize {
    let mut counts = HashMap::new();
    for (&(ch1, ch2), &freq) in freq_map {
        *counts.entry(ch1).or_default() += freq;
        *counts.entry(ch2).or_default() += freq;
    }

    counts
        .values_mut()
        .for_each(|freq| *freq = *freq / 2 + *freq % 2);

    let min = counts.values().copied().min().unwrap_or(0);
    let max = counts.values().copied().max().unwrap_or(0);

    max - min
}
