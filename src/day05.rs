use std::collections::HashMap;
use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day05.txt";

#[rustfmt::skip]
fn part1(input: &[((i64, i64), (i64, i64))]) -> Result<(), Error> {
    let mut map: HashMap<(i64, i64), bool> = HashMap::new();

    for &segment in input {
        let delta = (segment.1.0 - segment.0.0, segment.1.1 - segment.0.1);
        let delta = (
            delta.0.checked_div(delta.0.abs()).unwrap_or(0),
            delta.1.checked_div(delta.1.abs()).unwrap_or(0),
        );

        if delta.0 != 0 && delta.1 != 0 {
            continue;
        }

        let end = (segment.1.0 + delta.0, segment.1.1 + delta.1);
        let iter = std::iter::successors(Some(segment.0), |&it| {
            Some((it.0 + delta.0, it.1 + delta.1))
        })
        .take_while(|&it| it != end);

        for point in iter {
            map.entry(point).and_modify(|seen| *seen = true).or_insert(false);
        }
    }

    println!("part1: {}", map.into_values().filter(|&it| it).count());
    Ok(())
}

#[rustfmt::skip]
fn part2(input: &[((i64, i64), (i64, i64))]) -> Result<(), Error> {
    let mut map: HashMap<(i64, i64), bool> = HashMap::new();

    for &segment in input {
        let delta = (segment.1.0 - segment.0.0, segment.1.1 - segment.0.1);
        let delta = (
            delta.0.checked_div(delta.0.abs()).unwrap_or(0),
            delta.1.checked_div(delta.1.abs()).unwrap_or(0),
        );

        let end = (segment.1.0 + delta.0, segment.1.1 + delta.1);
        let iter = std::iter::successors(Some(segment.0), |&it| {
            Some((it.0 + delta.0, it.1 + delta.1))
        })
        .take_while(|&it| it != end);

        for point in iter {
            map.entry(point).and_modify(|seen| *seen = true).or_insert(false);
        }
    }

    println!("part2: {}", map.into_values().filter(|&it| it).count());
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Vec<((i64, i64), (i64, i64))> = input
        .lines()
        .map(|line| {
            let (p1, p2) = line
                .split_once("->")
                .ok_or_else(|| Error::msg("invalid input"))?;

            let (x1, y1) = p1
                .trim()
                .split_once(",")
                .ok_or_else(|| Error::msg("invalid input"))?;
            let (x2, y2) = p2
                .trim()
                .split_once(",")
                .ok_or_else(|| Error::msg("invalid input"))?;

            let (x1, y1) = (x1.parse()?, y1.parse()?);
            let (x2, y2) = (x2.parse()?, y2.parse()?);

            Ok(((x1, y1), (x2, y2)))
        })
        .collect::<Result<_, Error>>()?;

    part1(input.as_slice())?;
    part2(input.as_slice())?;

    Ok(())
}
