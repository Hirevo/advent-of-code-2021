use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day06.txt";

fn part1(input: &[usize]) -> Result<(), Error> {
    let mut groups = [0usize; 9];

    input.iter().for_each(|it| {
        groups[*it] += 1;
    });

    (0..80).for_each(|_| {
        groups.rotate_left(1);
        groups[6] += groups[8];
    });

    println!("part1: {}", groups.into_iter().sum::<usize>());
    Ok(())
}

fn part2(input: &[usize]) -> Result<(), Error> {
    let mut groups = [0usize; 9];

    input.iter().for_each(|it| {
        groups[*it] += 1;
    });

    (0..256).for_each(|_| {
        groups.rotate_left(1);
        groups[6] += groups[8];
    });

    println!("part2: {}", groups.into_iter().sum::<usize>());
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Vec<usize> = input
        .trim()
        .split(",")
        .map(|it| it.parse())
        .collect::<Result<_, _>>()?;

    part1(input.as_slice())?;
    part2(input.as_slice())?;

    Ok(())
}
