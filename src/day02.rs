use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day02.txt";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

fn part1(input: &[(Direction, usize)]) -> Result<(), Error> {
    let (hpos, depth) = input
        .iter()
        .fold((0, 0), |(hpos, depth), &(dir, value)| match dir {
            Direction::Forward => (hpos + value, depth),
            Direction::Down => (hpos, depth + value),
            Direction::Up => (hpos, depth - value),
        });

    println!("part1: {}", hpos * depth);
    Ok(())
}

fn part2(input: &[(Direction, usize)]) -> Result<(), Error> {
    let (hpos, depth, _) = input
        .iter()
        .fold((0, 0, 0), |(hpos, depth, aim), &(dir, value)| match dir {
            Direction::Forward => (hpos + value, depth + aim * value, aim),
            Direction::Down => (hpos, depth, aim + value),
            Direction::Up => (hpos, depth, aim - value),
        });

    println!("part2: {}", hpos * depth);
    Ok(())
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

    measured!(part1(commands.as_slice()))?;
    measured!(part2(commands.as_slice()))?;

    Ok(())
}
