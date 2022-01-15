use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use bitflags::bitflags;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day08.txt";

bitflags! {
    struct Wires: u8 {
        const A = 0b00000001;
        const B = 0b00000010;
        const C = 0b00000100;
        const D = 0b00001000;
        const E = 0b00010000;
        const F = 0b00100000;
        const G = 0b01000000;
    }
}

impl Wires {
    fn len(self) -> usize {
        // (0usize..7).map(|it| self.bits() >> it & 1 == 1).count()
        self.into_iter().count()
    }
}

impl IntoIterator for Wires {
    type Item = Wires;
    type IntoIter = WiresIter;

    fn into_iter(self) -> Self::IntoIter {
        WiresIter::new(self)
    }
}

struct WiresIter {
    wires: Wires,
    current: usize,
}

impl WiresIter {
    const WIRES: [Wires; 7] = [
        Wires::A,
        Wires::B,
        Wires::C,
        Wires::D,
        Wires::E,
        Wires::F,
        Wires::G,
    ];

    fn new(wires: Wires) -> WiresIter {
        WiresIter { wires, current: 0 }
    }
}

impl Iterator for WiresIter {
    type Item = Wires;

    fn next(&mut self) -> Option<Self::Item> {
        let mut found = None;
        while found.is_none() && self.current < Self::WIRES.len() {
            found = Some(Self::WIRES[self.current]).filter(|flag| self.wires.contains(*flag));
            self.current += 1;
        }
        found
    }
}

impl TryFrom<char> for Wires {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Wires::A),
            'b' => Ok(Wires::B),
            'c' => Ok(Wires::C),
            'd' => Ok(Wires::D),
            'e' => Ok(Wires::E),
            'f' => Ok(Wires::F),
            'g' => Ok(Wires::G),
            _ => Err(Error::msg("invalid input")),
        }
    }
}

fn part1(input: &[(Vec<Wires>, Vec<Wires>)]) -> Result<(), Error> {
    let answer = input
        .iter()
        .map(|(_, digits)| {
            digits
                .iter()
                .filter(|digit| matches!(digit.len(), 2 | 3 | 4 | 7))
                .count()
        })
        .sum::<usize>();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &[(Vec<Wires>, Vec<Wires>)]) -> Result<(), Error> {
    let answer = input
        .iter()
        .map(|(patterns, digits)| {
            let segments = infer_segment_mapping(patterns);
            let table = map_to_digit_mapping(&segments);

            digits
                .iter()
                .map(|digit| table.get(&digit).unwrap())
                .fold(0, |acc, digit| acc * 10 + digit)
        })
        .sum::<usize>();

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input: Vec<(Vec<Wires>, Vec<Wires>)> = input
        .trim()
        .lines()
        .map(|line| {
            let (patterns, digits) = line
                .split_once('|')
                .ok_or_else(|| Error::msg("invalid input"))?;

            let patterns = patterns
                .trim()
                .split(' ')
                .map(|chunk| {
                    chunk.trim().chars().try_fold(Wires::empty(), |acc, ch| {
                        let wire = Wires::try_from(ch)?;
                        Ok(acc | wire)
                    })
                })
                .collect::<Result<_, Error>>()?;

            let digits = digits
                .trim()
                .split(' ')
                .map(|chunk| {
                    chunk.trim().chars().try_fold(Wires::empty(), |acc, ch| {
                        let wire = Wires::try_from(ch)?;
                        Ok(acc | wire)
                    })
                })
                .collect::<Result<_, Error>>()?;

            Ok::<_, Error>((patterns, digits))
        })
        .collect::<Result<_, _>>()?;

    measured!(part1(input.as_slice()))?;
    measured!(part2(input.as_slice()))?;

    Ok(())
}

/// Given the patterns, analyses them and infers wire/segment mappings.
fn infer_segment_mapping(patterns: &[Wires]) -> [Wires; 7] {
    let mut segments = [Wires::empty(); 7];

    //? Easily identifiable cases, due to their unique length.
    let one = *patterns.iter().find(|pattern| pattern.len() == 2).unwrap();
    let four = *patterns.iter().find(|pattern| pattern.len() == 4).unwrap();
    let seven = *patterns.iter().find(|pattern| pattern.len() == 3).unwrap();
    let eight = *patterns.iter().find(|pattern| pattern.len() == 7).unwrap();

    // We create a set with the unknown digits.
    let unknown: HashSet<Wires> = patterns
        .iter()
        .copied()
        .filter(|it| ![one, four, seven, eight].contains(it))
        .collect();

    //? We can find segment 1 because only that segment differs between 1 and 7.
    segments[0] = seven.difference(one);

    //? We can find segment 6 and 3 from their unique frequency among all patterns.
    for segment in seven.intersection(one) {
        let count = patterns
            .iter()
            .filter(|pattern| !pattern.contains(segment))
            .count();

        match count {
            //? Segment 6 is only missing in one pattern among all (other than segment 1).
            1 if segment != segments[0] => segments[5] = segment,
            //? Segment 3 is only missing in two pattern among all.
            2 => segments[2] = segment,
            _ => {}
        }
    }

    //? We can find segment 4 and 7 because of their unique frequency among only unknown digits.
    for segment in Wires::all() {
        if segments.contains(&segment) {
            continue;
        }

        let count = unknown
            .iter()
            .filter(|pattern| !pattern.contains(segment))
            .count();

        match count {
            //? Segment 7 is never missing in any of the unknown digits.
            0 => segments[6] = segment,
            //? Segment 4 is only missing in one pattern among all.
            1 => segments[3] = segment,
            _ => {}
        }
    }

    //? We now know enough segments to single out segment 2 from the digit 4.
    segments[1] = four
        .into_iter()
        .find(|segment| !segments.contains(&segment))
        .unwrap();

    //? We can find segment 5 because it is the only one left.
    segments[4] = Wires::all()
        .into_iter()
        .find(|segment| !segments.contains(&segment))
        .unwrap();

    segments
}

/// Given the wire/segment mappings, computes the wire/digit mappings.
fn map_to_digit_mapping(segments: &[Wires; 7]) -> HashMap<Wires, usize> {
    //? This table represents which segments are enabled, for each digit from 0 through 9
    let table: [&[usize]; 10] = [
        &[0, 1, 2, 4, 5, 6],
        &[2, 5],
        &[0, 2, 3, 4, 6],
        &[0, 2, 3, 5, 6],
        &[1, 2, 3, 5],
        &[0, 1, 3, 5, 6],
        &[0, 1, 3, 4, 5, 6],
        &[0, 2, 5],
        &[0, 1, 2, 3, 4, 5, 6],
        &[0, 1, 2, 3, 5, 6],
    ];

    table
        .into_iter()
        .enumerate()
        .map(|(digit, indices)| {
            (
                indices
                    .into_iter()
                    .fold(Wires::empty(), |acc, idx| acc | segments[*idx]),
                digit,
            )
        })
        .collect()
}
