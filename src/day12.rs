use std::collections::HashMap;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day12.txt";

fn part1(map: &HashMap<String, Vec<String>>) -> Result<(), Error> {
    let mut stack = vec!["start"];
    let answer = search_part_1(map, &mut stack);

    println!("part1: {}", answer);
    Ok(())
}

fn part2(map: &HashMap<String, Vec<String>>) -> Result<(), Error> {
    let mut stack = vec!["start"];
    let answer = search_part_2(map, &mut stack, false);

    println!("part2: {}", answer);
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    // FIXME: switching to a string interner could make everything a lot faster (but it is already plenty fast).
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.trim().lines() {
        let (a, b) = line
            .trim()
            .split_once('-')
            .ok_or_else(|| Error::msg("invalid input"))?;

        map.entry(a.to_string()).or_default().push(b.to_string());
        map.entry(b.to_string()).or_default().push(a.to_string());
    }

    measured!(part1(&map))?;
    measured!(part2(&map))?;

    Ok(())
}

/// Basically a brute-force, we search through all non-disqualified
/// neighbours using recursion and count those that reaches the goal.
fn search_part_1<'a>(map: &'a HashMap<String, Vec<String>>, stack: &mut Vec<&'a str>) -> usize {
    let current = stack.last().copied().unwrap();
    if current == "end" {
        return 1;
    }

    let neighbours = map.get(current).unwrap().as_slice();

    let mut count = 0;
    for neighbour in neighbours {
        if neighbour.chars().all(|it| it.is_uppercase()) || !stack.contains(&neighbour.as_str()) {
            stack.push(neighbour.as_str());
            count += search_part_1(map, stack);
            stack.pop();
        }
    }

    count
}

/// Same approach as part 1, with the possibility of a single second chance added into it.
fn search_part_2<'a>(
    map: &'a HashMap<String, Vec<String>>,
    stack: &mut Vec<&'a str>,
    twice_used: bool,
) -> usize {
    let current = stack.last().copied().unwrap();
    if current == "end" {
        return 1;
    }

    let neighbours = map.get(current).unwrap().as_slice();

    let mut count = 0;
    for neighbour in neighbours {
        if neighbour.chars().all(|it| it.is_uppercase()) || !stack.contains(&neighbour.as_str()) {
            stack.push(neighbour.as_str());
            count += search_part_2(map, stack, twice_used);
            stack.pop();
        } else if !twice_used && neighbour != "start" && neighbour != "end" {
            stack.push(neighbour.as_str());
            count += search_part_2(map, stack, true);
            stack.pop();
        }
    }

    count
}
