use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day20.txt";

fn part1(pattern: &[char], image: &HashSet<(i64, i64)>) -> Result<(), Error> {
    let mut fst = HashSet::with_capacity(100000);
    let mut snd = HashSet::with_capacity(100000);

    fst.extend(image.iter().copied());

    let mut outer = 0;

    (0..2).for_each(|_| {
        step(pattern, &fst, &mut snd, outer);

        std::mem::swap(&mut fst, &mut snd);
        snd.clear();

        outer += 1;
        outer %= 2;
    });

    let answer = fst.len();

    println!("part1: {answer}");
    Ok(())
}

fn part2(pattern: &[char], image: &HashSet<(i64, i64)>) -> Result<(), Error> {
    let mut fst = HashSet::with_capacity(100000);
    let mut snd = HashSet::with_capacity(100000);

    fst.extend(image.iter().copied());

    let mut outer = 0;

    (0..50).for_each(|_| {
        step(pattern, &fst, &mut snd, outer);

        std::mem::swap(&mut fst, &mut snd);
        snd.clear();

        outer += 1;
        outer %= 2;
    });

    let answer = fst.len();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let (pattern, image) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| Error::msg("invalid input"))?;

    let pattern: Vec<char> = pattern.trim().chars().collect();
    let image: HashSet<(i64, i64)> = image
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect();

    measured!(part1(pattern.as_slice(), &image))?;
    measured!(part2(pattern.as_slice(), &image))?;

    Ok(())
}

fn step(
    pattern: &[char],
    image: &HashSet<(i64, i64)>,
    output: &mut HashSet<(i64, i64)>,
    outer: usize,
) {
    let (x_lo, x_hi) = image
        .iter()
        .copied()
        .fold((i64::MAX, i64::MIN), |(lo, hi), (x, _)| {
            (x.min(lo), x.max(hi))
        });
    let (y_lo, y_hi) = image
        .iter()
        .copied()
        .fold((i64::MAX, i64::MIN), |(lo, hi), (_, y)| {
            (y.min(lo), y.max(hi))
        });

    let (x_lo, x_hi) = (x_lo - 1, x_hi + 1);
    let (y_lo, y_hi) = (y_lo - 1, y_hi + 1);

    for y in y_lo..=y_hi {
        for x in x_lo..=x_hi {
            let mut index = 0;

            for_adjacents(x, y, |x_adj, y_adj| {
                let in_bounds = x_adj > x_lo && x_adj < x_hi && y_adj > y_lo && y_adj < y_hi;
                let bit = if in_bounds {
                    if image.contains(&(x_adj, y_adj)) {
                        1
                    } else {
                        0
                    }
                } else {
                    outer
                };

                index = (index << 1) + bit;
            });

            if pattern.get(index).copied() == Some('#') {
                output.insert((x, y));
            }
        }
    }
}

#[rustfmt::skip]
fn for_adjacents(x: i64, y: i64, mut func: impl FnMut(i64, i64)) {
    // top row
    func(x - 1, y - 1);
    func(x    , y - 1);
    func(x + 1, y - 1);

    // middle row
    func(x - 1, y);
    func(x    , y);
    func(x + 1, y);

    // bottom row
    func(x - 1, y + 1);
    func(x    , y + 1);
    func(x + 1, y + 1);
}
