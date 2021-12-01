use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day01.txt";

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let measures: Vec<usize> = input
        .lines()
        .map(|line| line.trim().parse())
        .collect::<Result<_, _>>()?;

    let part1 = measures.windows(2).filter(|w| w[0] < w[1]).count();
    println!("part1: {}", part1);

    let sums: Vec<usize> = measures.windows(3).map(|w| w.into_iter().sum()).collect();
    let part2 = sums.windows(2).filter(|w| w[0] < w[1]).count();
    println!("part2: {}", part2);

    Ok(())
}
