use std::fs;

use crate::Error;

pub const INPUT_PATH: &str = "inputs/day02.txt";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let commands: Vec<(Direction, usize)> = input
        .lines()
        .map(|line| {
            let (direction, value) = line
                .trim()
                .split_once(" ")
                .ok_or_else(|| Error::msg("invalid input"))?;

            let direction = match direction {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => {
                    return Err(Error::msg("unknown direction from input"));
                }
            };
            let value = value.parse()?;

            Ok((direction, value))
        })
        .collect::<Result<_, _>>()?;

    let part1 = commands
        .iter()
        .fold((0, 0), |(hpos, depth), &(dir, value)| match dir {
            Direction::Forward => (hpos + value, depth),
            Direction::Down => (hpos, depth + value),
            Direction::Up => (hpos, depth - value),
        });
    println!("part1: {}", part1.0 * part1.1);

    let part2 = commands
        .iter()
        .fold((0, 0, 0), |(hpos, depth, aim), &(dir, value)| match dir {
            Direction::Forward => (hpos + value, depth + aim * value, aim),
            Direction::Down => (hpos, depth, aim + value),
            Direction::Up => (hpos, depth, aim - value),
        });
    println!("part2: {}", part2.0 * part2.1);

    Ok(())
}
