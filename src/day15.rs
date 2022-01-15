use std::fs;

use pathfinding::directed::dijkstra;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day15.txt";

type Point = (usize, usize);

fn part1(input: &[u64], width: usize, height: usize) -> Result<(), Error> {
    let source: Point = (0, 0);
    let destination: Point = (width - 1, height - 1);

    let successors = |point: &Point| {
        let mut neighbours = Vec::with_capacity(4);

        for_around(width, height, *point, |(x, y)| {
            let cost = input[y * width + x];
            neighbours.push(((x, y), cost));
        });

        neighbours
    };

    let success = |point: &Point| *point == destination;

    let (_, answer) = dijkstra::dijkstra(&source, successors, success).expect("found no paths");

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &[u64], width: usize, height: usize) -> Result<(), Error> {
    let (new_width, new_height) = (width * 5, height * 5);

    let source: Point = (0, 0);
    let destination: Point = (new_width - 1, new_height - 1);

    let successors = |point: &Point| {
        let mut neighbours = Vec::with_capacity(4);

        for_around(new_width, new_height, *point, |(x, y)| {
            let mut cost = input[y % height * width + (x % width)];
            cost = cost + (y / height) as u64 + (x / width) as u64;
            cost = (cost - 1) % 9 + 1;
            neighbours.push(((x, y), cost));
        });

        neighbours
    };

    let success = |point: &Point| *point == destination;

    let (_, answer) = dijkstra::dijkstra(&source, successors, success).expect("found no paths");

    println!("part2: {answer}");
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

/// Calls the given function for all points around the specified one.
fn for_around(width: usize, height: usize, (x, y): Point, mut func: impl FnMut(Point)) {
    #[rustfmt::skip]
    let offsets: [(isize, isize); 4] = [
        (-1,  0),
        ( 0, -1),
        ( 0,  1),
        ( 1,  0),
    ];

    let (width, height) = (width as isize, height as isize);
    for (xoff, yoff) in offsets {
        let (nx, ny) = (x as isize + xoff, y as isize + yoff);
        if nx >= 0 && nx < width && ny >= 0 && ny < height {
            func((nx as usize, ny as usize));
        }
    }
}
