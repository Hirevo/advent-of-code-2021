use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day13.txt";

type Point = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instr {
    FoldX(usize),
    FoldY(usize),
}

fn part1(dots: &HashSet<Point>, instrs: &[Instr]) -> Result<(), Error> {
    let mut to = HashSet::with_capacity(dots.len());
    fold(&dots, &mut to, instrs[0]);

    let answer = to.len();
    println!("part1: {answer}");
    Ok(())
}

fn part2(dots: &HashSet<Point>, instrs: &[Instr]) -> Result<(), Error> {
    let mut from = dots.clone();
    let mut to = HashSet::with_capacity(dots.len());

    for instr in instrs.iter().copied() {
        fold(&from, &mut to, instr);
        std::mem::swap(&mut from, &mut to);
        to.clear();
    }

    let (w, h) = from
        .iter()
        .fold((0, 0), |(w, h), (x, y)| (w.max(*x), h.max(*y)));

    println!("part2:");

    for y in 0..=h {
        for x in 0..=w {
            if from.contains(&(x, y)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let (dots, instrs) = input
        .trim()
        .split_once("\n\n")
        .ok_or_else(|| Error::msg("invalid input"))?;

    let dots: HashSet<Point> = dots
        .lines()
        .map(|line| {
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| Error::msg("invalid input"))?;

            let x = x.parse()?;
            let y = y.parse()?;

            Ok((x, y))
        })
        .collect::<Result<_, Error>>()?;

    let instrs: Vec<Instr> = instrs
        .lines()
        .map(|line| {
            let (head, tail) = line
                .split_once('=')
                .ok_or_else(|| Error::msg("invalid input"))?;

            let value = tail.parse()?;
            match head.chars().next_back() {
                Some('x') => Ok(Instr::FoldX(value)),
                Some('y') => Ok(Instr::FoldY(value)),
                Some(_) | None => Err(Error::msg("invalid input")),
            }
        })
        .collect::<Result<_, Error>>()?;

    measured!(part1(&dots, instrs.as_slice()))?;
    measured!(part2(&dots, instrs.as_slice()))?;

    Ok(())
}

fn fold(from: &HashSet<Point>, to: &mut HashSet<Point>, instr: Instr) {
    for (x, y) in from.iter().copied() {
        let point = match instr {
            Instr::FoldX(value) if x > value => (value - (x - value), y),
            Instr::FoldY(value) if y > value => (x, value - (y - value)),
            _ => (x, y),
        };

        to.insert(point);
    }
}
