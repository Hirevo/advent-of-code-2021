use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day17.txt";

fn part1(_: i64, _: i64, min_y: i64, _: i64) -> Result<(), Error> {
    let answer = min_y.abs() * (min_y.abs() - 1) / 2;
    println!("part1: {answer}");
    Ok(())
}

fn part2(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Result<(), Error> {
    let mut answer = 0;

    for dy in (-min_y.abs())..=min_y.abs() {
        for dx in 0..=max_x {
            if ends_on_target(min_x, max_x, min_y, max_y, dx, dy) {
                answer += 1;
            }
        }
    }

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let (range_x, range_y) = input[13..]
        .trim()
        .split_once(", ")
        .ok_or_else(|| Error::msg("invalid input"))?;

    let (min_x, max_x) = range_x[2..]
        .trim()
        .split_once("..")
        .ok_or_else(|| Error::msg("invalid input"))?;
    let (min_y, max_y) = range_y[2..]
        .trim()
        .split_once("..")
        .ok_or_else(|| Error::msg("invalid input"))?;

    let (min_x, max_x) = (min_x.parse()?, max_x.parse()?);
    let (min_y, max_y) = (min_y.parse()?, max_y.parse()?);

    measured!(part1(min_x, max_x, min_y, max_y))?;
    measured!(part2(min_x, max_x, min_y, max_y))?;

    Ok(())
}

fn ends_on_target(
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    mut dx: i64,
    mut dy: i64,
) -> bool {
    let mut x = 0;
    let mut y = 0;

    while x < min_x || x > max_x || y < min_y || y > max_y {
        x += dx;
        y += dy;

        if (x < min_x && dx < 0) || (x > max_x && dx > 0) {
            return false;
        }
        if y < min_y && dy < 0 {
            return false;
        }

        dx -= 1 * dx.signum();
        dy -= 1;
    }

    true
}
