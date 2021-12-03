use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day01.txt";

fn part1(input: &[usize]) -> Result<(), Error> {
    let answer = input.windows(2).filter(|w| w[0] < w[1]).count();

    println!("part1: {}", answer);
    Ok(())
}

fn part2(input: &[usize]) -> Result<(), Error> {
    let sums: Vec<usize> = input.windows(3).map(|w| w.into_iter().sum()).collect();
    let answer = sums.windows(2).filter(|w| w[0] < w[1]).count();

    println!("part2: {}", answer);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let measures: Vec<usize> = input
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_, _>>()?;

    part1(measures.as_slice())?;
    part2(measures.as_slice())?;

    Ok(())
}
