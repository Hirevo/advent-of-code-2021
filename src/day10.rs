use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day10.txt";

fn part1(input: &[&str]) -> Result<(), Error> {
    let answer = input.iter().fold(0, |acc, line| {
        let mut stack = Vec::default();
        let error = line.chars().find_map(|ch| match ch {
            '(' | '{' | '[' | '<' => {
                stack.push(ch);
                None
            }
            ')' | '}' | ']' | '>' => {
                let expected = matching_character(ch);
                stack.pop().filter(|ch| *ch != expected).map(|_| ch)
            }
            _ => None,
        });

        error.map_or(acc, |error| match error {
            ')' => acc + 3,
            ']' => acc + 57,
            '}' => acc + 1197,
            '>' => acc + 25137,
            _ => acc,
        })
    });

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &[&str]) -> Result<(), Error> {
    let mut answer: Vec<usize> = input
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::default();
            let error = line.chars().find_map(|ch| match ch {
                '(' | '{' | '[' | '<' => {
                    stack.push(ch);
                    None
                }
                ')' | '}' | ']' | '>' => {
                    let expected = matching_character(ch);
                    stack.pop().filter(|ch| *ch != expected).map(|_| ch)
                }
                _ => None,
            });

            if error.is_some() {
                return None;
            }

            let mut score = 0;
            while let Some(ch) = stack.pop() {
                score *= 5;
                match ch {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => continue,
                }
            }

            Some(score)
        })
        .collect();

    let mid = answer.len() / 2;
    let (_, answer, _) = answer.select_nth_unstable(mid);

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Vec<&str> = input.trim().lines().collect();

    measured!(part1(input.as_slice()))?;
    measured!(part2(input.as_slice()))?;

    Ok(())
}

fn matching_character(ch: char) -> char {
    match ch {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        ')' => '(',
        '}' => '{',
        ']' => '[',
        '>' => '<',
        ch => ch,
    }
}
