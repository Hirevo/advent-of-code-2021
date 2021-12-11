#![allow(unreachable_code)]

use std::env;

use stable_eyre::eyre::Context;
use stable_eyre::Report;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod timing;

pub type Error = Report;

fn main() -> Result<(), Error> {
    stable_eyre::install()?;

    let arg: usize = match env::args().nth(1) {
        Some(arg) => arg.parse().context("could not parse arg as a day number")?,
        None => {
            eprintln!("please enter the day number you want to run.");
            std::process::exit(1);
        }
    };

    match arg {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        7 => day07::run(),
        8 => day08::run(),
        9 => day09::run(),
        10 => day10::run(),
        11 => day11::run(),
        n if (1..=24).contains(&n) => {
            println!("this day has not (yet) been solved !");
            Ok(())
        }
        _ => {
            eprintln!("invalid day number, it must be within the range [1,24].");
            std::process::exit(1);
        }
    }
}
