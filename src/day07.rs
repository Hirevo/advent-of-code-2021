use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day07.txt";

pub fn run() -> Result<(), Error> {
    let _input = fs::read_to_string(INPUT_PATH)?;

    todo!();

    Ok(())
}
