use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day09.txt";

fn part1(input: &[i64], width: usize, height: usize) -> Result<(), Error> {
    let low_points = find_low_points(input, width, height);

    let answer = low_points
        .iter()
        .map(|(x, y)| input[y * width + x] + 1)
        .sum::<i64>();

    println!("part1: {}", answer);
    Ok(())
}

fn part2(input: &[i64], width: usize, height: usize) -> Result<(), Error> {
    let low_points = find_low_points(input, width, height);

    let mut basin_lengths: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| {
            let mut seen = HashSet::new();
            flood_fill(input, *x, *y, width, height, &mut seen);
            seen.len()
        })
        .collect();

    basin_lengths.sort_unstable_by(|a, b| b.cmp(a));

    let answer = basin_lengths.iter().take(3).fold(1, |acc, it| acc * it);

    println!("part1: {}", answer);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let width = input
        .chars()
        .position(|it| it == '\n')
        .ok_or_else(|| Error::msg("invalid input"))?;
    let height = input.lines().filter(|it| !it.trim().is_empty()).count();

    let input: Vec<i64> = input
        .trim()
        .lines()
        .flat_map(|line| line.chars().map(|ch| ch.to_digit(10).map(|it| it as i64)))
        .collect::<Option<_>>()
        .ok_or_else(|| Error::msg("invalid input"))?;

    measured!(part1(input.as_slice(), width, height))?;
    measured!(part2(input.as_slice(), width, height))?;

    Ok(())
}

fn for_adjacents(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    mut func: impl FnMut(usize, usize),
) {
    if x > 0 {
        func(x - 1, y);
    }
    if x < width - 1 {
        func(x + 1, y);
    }
    if y > 0 {
        func(x, y - 1);
    }
    if y < height - 1 {
        func(x, y + 1);
    }
}

fn find_low_points(input: &[i64], width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut low_points = Vec::default();

    for y in 0..height {
        for x in 0..width {
            let value = input[y * width + x];
            let mut is_low_point = true;

            for_adjacents(x, y, width, height, |adj_x, adj_y| {
                let adj_value = input[adj_y * width + adj_x];
                is_low_point = is_low_point && (value < adj_value);
            });

            if is_low_point {
                low_points.push((x, y));
            }
        }
    }

    low_points
}

fn flood_fill(
    input: &[i64],
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    seen: &mut HashSet<(usize, usize)>,
) {
    if !seen.insert((x, y)) {
        return;
    }

    let value = input[y * width + x];
    for_adjacents(x, y, width, height, |adj_x, adj_y| {
        let adj_value = input[adj_y * width + adj_x];
        if value < adj_value && adj_value < 9 {
            flood_fill(input, adj_x, adj_y, width, height, seen);
        }
    });
}
