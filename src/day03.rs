use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day03.txt";

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let width = input
        .chars()
        .position(|it| it == '\n')
        .ok_or_else(|| Error::msg("invalid input"))?;

    let values: Vec<Vec<u8>> = input
        .lines()
        .map(|it| {
            it.chars()
                .flat_map(|it| char::to_digit(it, 2).map(|it| it as u8))
                .collect()
        })
        .collect();

    let (gamma_rate, epsilon_rate) = (0..width)
        .try_fold((0usize, 0usize), |(a, b), idx| {
            let count = values.iter().try_fold(0isize, |acc, it| {
                it.get(width - idx - 1)
                    .map(|&it| if it == 1 { acc + 1 } else { acc - 1 })
            })?;

            if count.is_positive() {
                Some((a + 2usize.pow(idx as u32), b))
            } else {
                Some((a, b + 2usize.pow(idx as u32)))
            }
        })
        .ok_or_else(|| Error::msg("invalid input"))?;

    println!("part1: {}", gamma_rate * epsilon_rate);

    let o2_rating =
        apply_criteria(&values, |count| count >= 0).ok_or_else(|| Error::msg("invalid input"))?;
    let co2_rating =
        apply_criteria(&values, |count| count < 0).ok_or_else(|| Error::msg("invalid input"))?;

    let o2_rating = o2_rating.iter().fold(0, |acc, &it| acc << 1 | it as usize);
    let co2_rating = co2_rating.iter().fold(0, |acc, &it| acc << 1 | it as usize);

    println!("part2: {}", o2_rating * co2_rating);

    Ok(())
}

fn apply_criteria(candidates: &[Vec<u8>], predicate: impl Fn(isize) -> bool) -> Option<&[u8]> {
    let mut head: usize = 0;
    let mut candidates: Vec<&[u8]> = candidates.iter().map(|it| it.as_slice()).collect();

    while candidates.len() > 1 {
        let count = candidates.iter().try_fold(0isize, |acc, it| {
            it.get(head)
                .map(|&it| if it == 1 { acc + 1 } else { acc - 1 })
        })?;

        let value = match predicate(count) {
            true => 1,
            false => 0,
        };

        candidates.retain(|it| it.get(head).map_or(false, |&it| it == value));

        head += 1;
    }

    candidates.get(0).map(|&it| it)
}
