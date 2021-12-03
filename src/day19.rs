use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day19.txt";

fn part1(_input: &str) -> Result<(), Error> {
    todo!()
}

fn part2(_input: &str) -> Result<(), Error> {
    todo!();
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    part1(input.as_str())?;
    part2(input.as_str())?;

    Ok(())
}
