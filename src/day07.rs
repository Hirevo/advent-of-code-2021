use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day07.txt";

fn part1(input: &[i64]) -> Result<(), Error> {
    let mut data = input.to_vec();
    data.sort_unstable();

    let median = if data.len() % 2 == 0 {
        data[data.len() / 2]
    } else {
        let mid = data.len() / 2;
        (data[mid] + data[mid + 1]) / 2
    };

    let fuel = data.iter().map(|it| (median - it).abs()).sum::<i64>();

    println!("part1: {}", fuel);
    Ok(())
}

fn sum_up_to(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn part2(input: &[i64]) -> Result<(), Error> {
    let average = input.iter().sum::<i64>() / input.len() as i64;

    let p1 = input
        .iter()
        .map(|it| sum_up_to((average - it).abs()))
        .sum::<i64>();
    let p2 = input
        .iter()
        .map(|it| sum_up_to((average - it + 1).abs()))
        .sum::<i64>();

    println!("part2: {}", p1.min(p2));
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Vec<i64> = input
        .trim()
        .split(",")
        .map(|it| it.parse())
        .collect::<Result<_, _>>()?;

    measured!(part1(input.as_slice()))?;
    measured!(part2(input.as_slice()))?;

    Ok(())
}
