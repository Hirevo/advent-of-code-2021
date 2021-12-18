use std::fmt;
use std::fs;
use std::ops::Add;

use crate::measured;
use crate::parser::combinators::{between, some};
use crate::parser::Parser;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day18.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Number(i64),
    Pair(Box<Tree>, Box<Tree>),
}

impl Add for Tree {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Tree::Pair(Box::new(self), Box::new(rhs));
        result.reduce();
        result
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Pair(left, right) => write!(f, "[{},{}]", left, right),
        }
    }
}

struct ExplodeResult {
    left: Option<i64>,
    right: Option<i64>,
    just_emitted: bool,
}

impl Tree {
    fn is_number(&self) -> bool {
        matches!(self, Tree::Number(_))
    }

    fn as_number(&self) -> Option<i64> {
        match self {
            Self::Number(n) => Some(*n),
            _ => None,
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Self::Number(n) => *n,
            Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    pub fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, depth: usize) -> Option<ExplodeResult> {
        if let Tree::Pair(left, right) = self {
            if let Some(mut result) = left.explode(depth + 1) {
                if result.just_emitted {
                    *left = Box::new(Tree::Number(0));
                }

                result.just_emitted = false;

                if let Some(value) = result.right {
                    if right.add_to_leftmost(value) {
                        result.right = None;
                    }
                }

                return Some(result);
            }

            if depth >= 4 && left.is_number() && right.is_number() {
                return Some(ExplodeResult {
                    left: left.as_number(),
                    right: right.as_number(),
                    just_emitted: true,
                });
            }

            if let Some(mut result) = right.explode(depth + 1) {
                if result.just_emitted {
                    *right = Box::new(Tree::Number(0));
                }

                result.just_emitted = false;

                if let Some(value) = result.left {
                    if left.add_to_rightmost(value) {
                        result.left = None;
                    }
                }

                return Some(result);
            }
        }

        None
    }

    fn split(&mut self) -> bool {
        if let Tree::Pair(left, right) = self {
            if let Some(number) = left.as_number() {
                if number > 9 {
                    *left = Box::new(Self::Pair(
                        Box::new(Self::Number(number / 2)),
                        Box::new(Self::Number(number / 2 + number % 2)),
                    ));
                    return true;
                }
            } else {
                if left.split() {
                    return true;
                }
            }

            if let Some(number) = right.as_number() {
                if number > 9 {
                    *right = Box::new(Self::Pair(
                        Box::new(Self::Number(number / 2)),
                        Box::new(Self::Number(number / 2 + number % 2)),
                    ));
                    return true;
                }
            } else {
                if right.split() {
                    return true;
                }
            }
        }

        false
    }

    fn add_to_leftmost(&mut self, value: i64) -> bool {
        match self {
            Self::Number(n) => {
                *n += value;
                true
            }
            Self::Pair(left, _) => left.add_to_leftmost(value),
        }
    }

    fn add_to_rightmost(&mut self, value: i64) -> bool {
        match self {
            Self::Number(n) => {
                *n += value;
                true
            }
            Self::Pair(_, right) => right.add_to_rightmost(value),
        }
    }
}

fn part1(numbers: &[Tree]) -> Result<(), Error> {
    let answer = numbers.iter().cloned().reduce(|acc, it| acc + it).unwrap();
    println!("part1: {}", answer.magnitude());
    Ok(())
}

fn part2(numbers: &[Tree]) -> Result<(), Error> {
    let mut answer = 0;

    for i1 in 0..numbers.len() {
        for i2 in 0..numbers.len() {
            if i1 != i2 {
                let result = numbers[i1].clone() + numbers[i2].clone();
                answer = answer.max(result.magnitude());
            }
        }
    }

    println!("part2: {}", answer);
    Ok(())
}

fn exact<'a>(ch: char) -> impl Parser<char, &'a [char]> {
    move |input: &'a [char]| {
        let (head, rest) = input.split_first()?;
        (ch == *head).then(|| (ch, rest))
    }
}

fn digit<'a>() -> impl Parser<i64, &'a [char]> {
    move |input: &'a [char]| {
        let (head, rest) = input.split_first()?;
        head.to_digit(10).map(|digit| (digit as i64, rest))
    }
}

fn number<'a>() -> impl Parser<Tree, &'a [char]> {
    some(digit())
        .map(|digits| digits.into_iter().fold(0, |acc, it| acc * 10 + it))
        .map(Tree::Number)
}

fn pair<'a>() -> impl Parser<Tree, &'a [char]> {
    between(
        exact('['),
        tree().and_left(exact(',')).and(tree()),
        exact(']'),
    )
    .map(|(left, right)| Tree::Pair(Box::new(left), Box::new(right)))
}

fn eoi<'a>() -> impl Parser<(), &'a [char]> {
    move |input: &'a [char]| input.is_empty().then(|| ((), input))
}

fn tree<'a>() -> impl Parser<Tree, &'a [char]> {
    move |input: &'a [char]| pair().or(number()).parse(input)
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let numbers: Vec<Tree> = input
        .trim()
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.trim().chars().collect();
            let (tree, _) = tree().and_left(eoi()).parse(chars.as_slice())?;
            Some(tree)
        })
        .collect::<Option<_>>()
        .ok_or_else(|| Error::msg("invalid input"))?;

    measured!(part1(&numbers))?;
    measured!(part2(&numbers))?;

    Ok(())
}
