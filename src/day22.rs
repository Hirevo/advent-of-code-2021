use std::collections::HashSet;
use std::fs;

use crate::measured;
use crate::parser::combinators::{alternative, optional, some};
use crate::parser::Parser;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day22.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    op: Operation,
    cuboid: Cuboid,
}

impl Step {
    fn new(op: Operation, cuboid: Cuboid) -> Self {
        Self { op, cuboid }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cuboid {
    lo: Point,
    hi: Point,
}

impl Cuboid {
    pub fn new(lo: Point, hi: Point) -> Self {
        Self { lo, hi }
    }

    pub fn is_empty(&self) -> bool {
        !(self.hi.x > self.lo.x && self.hi.y > self.lo.y && self.hi.z > self.lo.z)
    }

    pub fn volume(&self) -> i64 {
        (self.hi.x - self.lo.x) * (self.hi.y - self.lo.y) * (self.hi.z - self.lo.z)
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let new_lo = Point::new(
            self.lo.x.max(other.lo.x),
            self.lo.y.max(other.lo.y),
            self.lo.z.max(other.lo.z),
        );

        let new_hi = Point::new(
            self.hi.x.min(other.hi.x),
            self.hi.y.min(other.hi.y),
            self.hi.z.min(other.hi.z),
        );

        let cuboid = Cuboid::new(new_lo, new_hi);

        Some(cuboid).filter(|it| !it.is_empty())
    }

    #[rustfmt::skip]
    pub fn split(&self, other: &Self) -> Option<Vec<Self>> {
        let mut cuboids = Vec::new();

        let intersection = self.intersection(other)?;

        if *self == intersection {
            return Some(Vec::default());
        }

        // TODO(hirevo): actual implementation.

        // LEFT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            intersection.lo.y,
            intersection.lo.z,
        );
        let new_hi = Point::new(
            intersection.lo.x,
            intersection.hi.y,
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // RIGHT
        let new_lo = Point::new(
            intersection.hi.x,
            intersection.lo.y,
            intersection.lo.z,
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            intersection.hi.y,
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP
        let new_lo = Point::new(
            intersection.lo.x,
            self.lo.y.min(intersection.lo.y),
            intersection.lo.z,
        );
        let new_hi = Point::new(
            intersection.hi.x,
            intersection.lo.y,
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM
        let new_lo = Point::new(
            intersection.lo.x,
            intersection.hi.y,
            intersection.lo.z,
        );
        let new_hi = Point::new(
            intersection.hi.x,
            self.hi.y.max(intersection.hi.y),
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // FRONT
        let new_lo = Point::new(
            intersection.lo.x,
            intersection.lo.y,
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            intersection.hi.x,
            intersection.hi.y,
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BACK
        let new_lo = Point::new(
            intersection.lo.x,
            intersection.lo.y,
            intersection.hi.z,
        );
        let new_hi = Point::new(
            intersection.hi.x,
            intersection.hi.y,
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-LEFT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            self.lo.y.min(intersection.lo.y),
            intersection.lo.z,
        );
        let new_hi = Point::new(
            intersection.lo.x,
            intersection.lo.y,
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-RIGHT
        let new_lo = Point::new(
            intersection.hi.x,
            self.lo.y.min(intersection.lo.y),
            intersection.lo.z,
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            intersection.lo.y,
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-LEFT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            intersection.hi.y,
            intersection.lo.z,
        );
        let new_hi = Point::new(
            intersection.lo.x,
            self.hi.y.max(intersection.hi.y),
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-RIGHT
        let new_lo = Point::new(
            intersection.hi.x,
            intersection.hi.y,
            intersection.lo.z,
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            self.hi.y.max(intersection.hi.y),
            intersection.hi.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // FRONT-LEFT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            intersection.lo.y,
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            intersection.lo.x,
            intersection.hi.y,
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // FRONT-RIGHT
        let new_lo = Point::new(
            intersection.hi.x,
            intersection.lo.y,
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            intersection.hi.y,
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BACK-LEFT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            intersection.lo.y,
            intersection.hi.z,
        );
        let new_hi = Point::new(
            intersection.lo.x,
            intersection.hi.y,
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BACK-RIGHT
        let new_lo = Point::new(
            intersection.hi.x,
            intersection.lo.y,
            intersection.hi.z,
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            intersection.hi.y,
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-FRONT
        let new_lo = Point::new(
            intersection.lo.x,
            self.lo.y.min(intersection.lo.y),
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            intersection.hi.x,
            intersection.lo.y,
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-BACK
        let new_lo = Point::new(
            intersection.lo.x,
            self.lo.y.min(intersection.lo.y),
            intersection.hi.z,
        );
        let new_hi = Point::new(
            intersection.hi.x,
            intersection.lo.y,
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-FRONT
        let new_lo = Point::new(
            intersection.lo.x,
            intersection.hi.y,
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            intersection.hi.x,
            self.hi.y.max(intersection.hi.y),
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-BACK
        let new_lo = Point::new(
            intersection.lo.x,
            intersection.hi.y,
            intersection.hi.z,
        );
        let new_hi = Point::new(
            intersection.hi.x,
            self.hi.y.max(intersection.hi.y),
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-LEFT-FRONT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            self.lo.y.min(intersection.lo.y),
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            intersection.lo.x,
            intersection.lo.y,
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-LEFT-BACK
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            self.lo.y.min(intersection.lo.y),
            intersection.hi.z,
        );
        let new_hi = Point::new(
            intersection.lo.x,
            intersection.lo.y,
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-RIGHT-FRONT
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            intersection.hi.y,
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            intersection.lo.x,
            self.hi.y.max(intersection.hi.y),
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // TOP-RIGHT-BACK
        let new_lo = Point::new(
            self.lo.x.min(intersection.lo.x),
            intersection.hi.y,
            intersection.hi.z,
        );
        let new_hi = Point::new(
            intersection.lo.x,
            self.hi.y.max(intersection.hi.y),
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-LEFT-FRONT
        let new_lo = Point::new(
            intersection.hi.x,
            self.lo.y.min(intersection.lo.y),
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            intersection.lo.y,
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-LEFT-BACK
        let new_lo = Point::new(
            intersection.hi.x,
            self.lo.y.min(intersection.lo.y),
            intersection.hi.z,
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            intersection.lo.y,
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-RIGHT-FRONT
        let new_lo = Point::new(
            intersection.hi.x,
            intersection.hi.y,
            self.lo.z.min(intersection.lo.z),
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            self.hi.y.max(intersection.hi.y),
            intersection.lo.z,
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        // BOTTOM-RIGHT-BACK
        let new_lo = Point::new(
            intersection.hi.x,
            intersection.hi.y,
            intersection.hi.z,
        );
        let new_hi = Point::new(
            self.hi.x.max(intersection.hi.x),
            self.hi.y.max(intersection.hi.y),
            self.hi.z.max(intersection.hi.z),
        );
        let cuboid = Cuboid::new(new_lo, new_hi);
        if !cuboid.is_empty() {
            cuboids.push(cuboid);
        }

        Some(cuboids)
    }
}

// This is the bruteforce method.
// We just have a set of every single 3D point that is currently on, and update it for each step.
fn part1(steps: &[Step]) -> Result<(), Error> {
    let mut set = HashSet::new();

    for step in steps {
        // NOTE: our cuboid bounds are exclusive for the `hi` point, so we use 51 instead of 50.
        let in_bounds = (step.cuboid.lo.x >= -50)
            && (step.cuboid.hi.x <= 51)
            && (step.cuboid.lo.y >= -50)
            && (step.cuboid.hi.y <= 51)
            && (step.cuboid.lo.z >= -50)
            && (step.cuboid.hi.z <= 51);

        if !in_bounds {
            continue;
        }

        for x in (step.cuboid.lo.x)..(step.cuboid.hi.x) {
            for y in (step.cuboid.lo.y)..(step.cuboid.hi.y) {
                for z in (step.cuboid.lo.z)..(step.cuboid.hi.z) {
                    match step.op {
                        Operation::On => {
                            set.insert(Point::new(x, y, z));
                        }
                        Operation::Off => {
                            set.remove(&Point::new(x, y, z));
                        }
                    }
                }
            }
        }
    }

    let answer = set.len();

    println!("part1: {answer}");
    Ok(())
}

// This is the smarter method.
// We subdivide into smaller cuboids the volume of the difference between two cuboids after intersecting,
// and we keep a list of the cuboids that are currently on (which keeps that list rather small).
fn part2(steps: &[Step]) -> Result<(), Error> {
    let mut cuboids: Vec<Cuboid> = Vec::new();

    for step in steps {
        match step.op {
            Operation::On => {
                let mut fst = Vec::new();
                let mut snd = Vec::new();
                fst.push(step.cuboid.clone());

                for it1 in &cuboids {
                    for it2 in &fst {
                        match it2.split(&it1) {
                            Some(mut splitted) => {
                                snd.append(&mut splitted);
                            }
                            None => {
                                snd.push(it2.clone());
                            }
                        }
                    }

                    std::mem::swap(&mut fst, &mut snd);
                    snd.clear();
                }

                cuboids.append(&mut fst);
            }
            Operation::Off => {
                let mut new_cuboids = Vec::with_capacity(cuboids.len());

                for it in cuboids {
                    match it.split(&step.cuboid) {
                        Some(mut splitted) => {
                            new_cuboids.append(&mut splitted);
                        }
                        None => {
                            new_cuboids.push(it.clone());
                        }
                    }
                }

                cuboids = new_cuboids;
            }
        }
    }

    let answer: i64 = cuboids.into_iter().map(|it| it.volume()).sum();

    println!("part2: {answer}");
    Ok(())
}

fn exact<'a>(ch: char) -> impl Parser<char, &'a [char]> {
    move |input: &'a [char]| {
        let (head, rest) = input.split_first()?;
        (ch == *head).then(|| (ch, rest))
    }
}

fn exact_str<'a>(string: &'a str) -> impl Parser<&'a str, &'a [char]> {
    move |input: &'a [char]| {
        let (head, rest) = input.split_at(string.len());
        string
            .chars()
            .eq(head.iter().copied())
            .then(|| (string, rest))
    }
}

fn operation<'a>() -> impl Parser<Operation, &'a [char]> {
    alternative(
        exact_str("on").map(|_| Operation::On),
        exact_str("off").map(|_| Operation::Off),
    )
}

fn digit<'a>() -> impl Parser<i64, &'a [char]> {
    move |input: &'a [char]| {
        let (head, rest) = input.split_first()?;
        head.to_digit(10).map(|digit| (digit as i64, rest))
    }
}

fn number<'a>() -> impl Parser<i64, &'a [char]> {
    optional(exact('-'))
        .and(some(digit()))
        .map(|(sign, digits)| {
            let number = digits.into_iter().fold(0, |acc, it| acc * 10 + it);
            sign.is_some().then(|| -number).unwrap_or_else(|| number)
        })
}

fn interval<'a>() -> impl Parser<(i64, i64), &'a [char]> {
    number().and_left(exact_str("..")).and(number())
}

fn cuboid<'a>() -> impl Parser<Cuboid, &'a [char]> {
    exact_str("x=")
        .and_right(interval())
        .and_left(exact_str(",y="))
        .and(interval())
        .and_left(exact_str(",z="))
        .and(interval())
        .map(|((x, y), z)| {
            let lo = Point::new(x.0, y.0, z.0);
            let hi = Point::new(x.1 + 1, y.1 + 1, z.1 + 1);
            Cuboid::new(lo, hi)
        })
}

fn step<'a>() -> impl Parser<Step, &'a [char]> {
    operation()
        .and_left(exact(' '))
        .and(cuboid())
        .map(|(instr, cuboid)| Step::new(instr, cuboid))
}

fn eoi<'a>() -> impl Parser<(), &'a [char]> {
    move |input: &'a [char]| input.is_empty().then(|| ((), input))
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let step: Vec<Step> = input
        .trim()
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.trim().chars().collect();
            let (operation, _) = step().and_left(eoi()).parse(chars.as_slice())?;
            Some(operation)
        })
        .collect::<Option<_>>()
        .ok_or_else(|| Error::msg("invalid input"))?;

    measured!(part1(step.as_slice()))?;
    measured!(part2(step.as_slice()))?;

    Ok(())
}
