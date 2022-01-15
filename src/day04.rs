use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day04.txt";

fn part1(deck: &[usize], boards: &[Board]) -> Result<(), Error> {
    let (board, marked) =
        find_first_winning_board(deck, boards).ok_or_else(|| Error::msg("invalid input"))?;

    let score = board.compute_score(marked);

    println!("part1: {score}");
    Ok(())
}

fn part2(deck: &[usize], boards: &[Board]) -> Result<(), Error> {
    let (board, marked) =
        find_last_winning_board(deck, boards).ok_or_else(|| Error::msg("invalid input"))?;

    let score = board.compute_score(marked);

    println!("part2: {score}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let mut iter = input.split("\n\n");
    let deck: Vec<usize> = {
        let line = iter.next().ok_or_else(|| Error::msg("invalid input"))?;
        line.split(',')
            .map(|it| it.trim().parse())
            .collect::<Result<_, _>>()?
    };
    let board: Vec<Board> = iter
        .map(|board| Board::from_input(board))
        .collect::<Result<_, _>>()?;

    measured!(part1(deck.as_slice(), board.as_slice()))?;
    measured!(part2(deck.as_slice(), board.as_slice()))?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    width: usize,
    height: usize,
    values: Vec<usize>,
}

impl Board {
    pub fn from_input(board: &str) -> Result<Self, Error> {
        let height = board.lines().count();

        let values: Vec<usize> = board
            .lines()
            .flat_map(|it| it.split_whitespace())
            .map(|it| it.trim().parse())
            .collect::<Result<_, _>>()?;

        let width = values.len() / height;

        Ok(Self {
            values,
            width,
            height,
        })
    }

    fn is_won(&self, marked: &[usize]) -> bool {
        let row_win = || {
            self.values
                .chunks_exact(self.width)
                .any(|it| it.iter().all(|it| marked.contains(it)))
        };

        let column_win = || {
            (0..self.height).any(|idx| {
                self.values
                    .iter()
                    .skip(idx)
                    .step_by(self.width)
                    .all(|it| marked.contains(it))
            })
        };

        row_win() || column_win()
    }

    fn compute_score(&self, marked: &[usize]) -> usize {
        let last = marked.last().copied().unwrap_or(1);
        let sum: usize = self.values.iter().filter(|it| !marked.contains(it)).sum();
        sum * last
    }
}

fn find_first_winning_board<'a, 'b>(
    deck: &'a [usize],
    boards: &'b [Board],
) -> Option<(&'b Board, &'a [usize])> {
    (0..deck.len()).map(|idx| &deck[..idx]).find_map(|drawn| {
        boards
            .iter()
            .find(|board| board.is_won(drawn))
            .map(|board| (board, drawn))
    })
}

fn find_last_winning_board<'a, 'b>(
    deck: &'a [usize],
    boards: &'b [Board],
) -> Option<(&'b Board, &'a [usize])> {
    let mut candidates: Vec<&Board> = boards.iter().collect();

    (0..deck.len()).map(|idx| &deck[..idx]).find_map(|drawn| {
        let last = candidates.first().map(|board| *board);

        candidates.retain(|board| !board.is_won(drawn));

        last.filter(|_| candidates.is_empty())
            .map(|board| (board, drawn))
    })
}
