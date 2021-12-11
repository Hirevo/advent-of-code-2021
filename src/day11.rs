use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day11.txt";

type Point = (usize, usize);

fn part1(input: &[u64], width: usize, height: usize) -> Result<(), Error> {
    let mut grid = input.to_vec();

    let answer: usize = (0..100).map(|_| tick(&mut grid, width, height)).sum();

    println!("part1: {}", answer);
    Ok(())
}

fn part2(input: &[u64], width: usize, height: usize) -> Result<(), Error> {
    let size = width * height;
    let mut grid = input.to_vec();

    let answer = (1..)
        .find(|_| tick(&mut grid, width, height) == size)
        .unwrap();

    println!("part2: {}", answer);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let width = input
        .chars()
        .position(|it| it == '\n')
        .ok_or_else(|| Error::msg("invalid input"))?;
    let height = input.lines().filter(|it| !it.trim().is_empty()).count();

    let input: Vec<u64> = input
        .chars()
        .filter(|it| it.is_numeric())
        .map(|it| char::to_digit(it, 10).map(u64::from))
        .collect::<Option<_>>()
        .ok_or_else(|| Error::msg("invalid input"))?;

    measured!(part1(input.as_slice(), width, height))?;
    measured!(part2(input.as_slice(), width, height))?;

    Ok(())
}

/// Simulates one complete turn and returns the number of flashes that happened for that turn.
fn tick(grid: &mut [u64], width: usize, height: usize) -> usize {
    let mut flashed = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            bump(grid, width, height, (x, y), &mut flashed);
        }
    }

    for (x, y) in flashed.iter().copied() {
        grid[y * width + x] = 0;
    }

    flashed.len()
}

/// Bumps by one the current point and recurses to bump those around, if a flash occurs.
fn bump(
    grid: &mut [u64],
    width: usize,
    height: usize,
    (x, y): Point,
    flashed: &mut HashSet<Point>,
) {
    let value = grid[y * width + x] + 1;
    grid[y * width + x] = value;

    if value > 9 && !flashed.contains(&(x, y)) {
        flashed.insert((x, y));
        for_around(width, height, (x, y), |(x, y)| {
            bump(grid, width, height, (x, y), flashed);
        });
    }
}

/// Calls the given function for all points around the specified one.
fn for_around(width: usize, height: usize, (x, y): Point, mut func: impl FnMut(Point)) {
    #[rustfmt::skip]
    let offsets: [(isize, isize); 8] = [
        (-1, -1),
        (-1,  0),
        (-1,  1),
        ( 0, -1),
        ( 0,  1),
        ( 1, -1),
        ( 1,  0),
        ( 1,  1),
    ];

    let (width, height) = (width as isize, height as isize);
    for (xoff, yoff) in offsets {
        let (nx, ny) = (x as isize + xoff, y as isize + yoff);
        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            func((nx as usize, ny as usize));
        }
    }
}
