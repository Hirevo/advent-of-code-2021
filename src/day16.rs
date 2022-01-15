use std::collections::VecDeque;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day16.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketKind {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(u64),
    GreaterThan(Box<Packet>, Box<Packet>),
    LessThan(Box<Packet>, Box<Packet>),
    Equal(Box<Packet>, Box<Packet>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    kind: PacketKind,
    version: u64,
}

impl Packet {
    pub fn from_bits(bits: &mut VecDeque<u8>) -> Option<Self> {
        fn parse_integer(bits: &mut VecDeque<u8>, size: usize) -> Option<u64> {
            (0..size).try_fold(0u64, |acc, _| {
                bits.pop_front().map(|bit| (acc << 1) + bit as u64)
            })
        }

        fn parse_literal(bits: &mut VecDeque<u8>) -> Option<u64> {
            let mut value = 0;
            let mut stop = false;

            while !stop {
                stop = bits.pop_front()? == 0;
                for _ in 0..4 {
                    value = (value << 1) + bits.pop_front()? as u64;
                }
            }

            Some(value)
        }

        let version = parse_integer(bits, 3)?;
        let type_id = parse_integer(bits, 3)?;

        if type_id == 4 {
            let literal = parse_literal(bits)?;
            let kind = PacketKind::Literal(literal);
            return Some(Packet { version, kind });
        }

        let length_type_id = parse_integer(bits, 1)?;

        let subpackets: Vec<Packet> = match length_type_id {
            0 => {
                let length = parse_integer(bits, 15)? as usize;

                let start = bits.len();
                let mut subpackets = Vec::default();
                while start - bits.len() < length {
                    let packet = Packet::from_bits(bits)?;
                    subpackets.push(packet);
                }

                subpackets
            }
            1 => {
                let length = parse_integer(bits, 11)?;
                (0..length)
                    .map(|_| Packet::from_bits(bits))
                    .collect::<Option<_>>()?
            }
            _ => {
                return None;
            }
        };

        let kind = match type_id {
            0 => PacketKind::Sum(subpackets),
            1 => PacketKind::Product(subpackets),
            2 => PacketKind::Min(subpackets),
            3 => PacketKind::Max(subpackets),
            4 => unreachable!(),
            5 => {
                let mut iter = subpackets.into_iter().map(Box::new);
                PacketKind::GreaterThan(iter.next()?, iter.next()?)
            }
            6 => {
                let mut iter = subpackets.into_iter().map(Box::new);
                PacketKind::LessThan(iter.next()?, iter.next()?)
            }
            7 => {
                let mut iter = subpackets.into_iter().map(Box::new);
                PacketKind::Equal(iter.next()?, iter.next()?)
            }
            _ => unreachable!(),
        };

        Some(Packet { version, kind })
    }

    #[rustfmt::skip]
    fn sum_versions(&self) -> u64 {
        match &self.kind {
            PacketKind::Literal(_) => self.version,
            PacketKind::Sum(subpackets)
            | PacketKind::Product(subpackets)
            | PacketKind::Min(subpackets)
            | PacketKind::Max(subpackets) =>
                self.version + subpackets.iter().map(|it| it.sum_versions()).sum::<u64>(),
            PacketKind::GreaterThan(lhs, rhs)
            | PacketKind::LessThan(lhs, rhs)
            | PacketKind::Equal(lhs, rhs) =>
                self.version + lhs.sum_versions() + rhs.sum_versions(),
        }
    }

    #[rustfmt::skip]
    fn evaluate(&self) -> u64 {
        match &self.kind {
            PacketKind::Sum(subpackets) => subpackets.iter().fold(0, |acc, it| acc + it.evaluate()),
            PacketKind::Product(subpackets) => subpackets.iter().fold(1, |acc, it| acc * it.evaluate()),
            PacketKind::Min(subpackets) => subpackets.iter().map(|it| it.evaluate()).min().expect("minimum of no value"),
            PacketKind::Max(subpackets) => subpackets.iter().map(|it| it.evaluate()).max().expect("maximum of no value"),
            PacketKind::Literal(value) => *value,
            PacketKind::GreaterThan(lhs, rhs) => if lhs.evaluate() > rhs.evaluate() { 1 } else { 0 },
            PacketKind::LessThan(lhs, rhs) => if lhs.evaluate() < rhs.evaluate() { 1 } else { 0 },
            PacketKind::Equal(lhs, rhs) => if lhs.evaluate() == rhs.evaluate() { 1 } else { 0 },
        }
    }
}

fn part1(packet: &Packet) -> Result<(), Error> {
    let answer = packet.sum_versions();
    println!("part1: {answer}");
    Ok(())
}

fn part2(packet: &Packet) -> Result<(), Error> {
    let answer = packet.evaluate();
    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let mut bits: VecDeque<u8> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(16).map(to_binary))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| Error::msg("invalid input"))?
        .into_iter()
        .flatten()
        .collect();

    let packet = Packet::from_bits(&mut bits).ok_or_else(|| Error::msg("invalid input"))?;

    measured!(part1(&packet))?;
    measured!(part2(&packet))?;

    Ok(())
}

fn to_binary(mut digit: u32) -> [u8; 4] {
    [8, 4, 2, 1].map(|factor| {
        if digit >= factor {
            digit -= factor;
            1
        } else {
            0
        }
    })
}
