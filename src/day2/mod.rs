use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

use crate::{solution::Solution, util::split_parse};

pub struct Day2;

#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

use Shape::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Round {
    you: Shape,
    other: Shape,
}

type Day2Input = Vec<Round>;

impl FromStr for Round {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (str0, str1) = s
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| anyhow!("bad input string: {:?}", s))?;

        let shape0 = match str0 {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => bail!("unexpected string for shape0: {:?}, {:?}", str0, s),
        };

        let shape1 = match str1 {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => bail!("unexpected string for shape1: {:?}, {:?}", str1, s),
        };

        Ok(Round {
            other: shape0,
            you: shape1,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RoundOutcome {
    Win,
    Loss,
    Draw,
}
use RoundOutcome::*;

fn get_round_outcome(round: &Round) -> RoundOutcome {
    if round.you == round.other {
        Draw
    } else if matches!(
        round,
        Round {
            you: Rock,
            other: Scissors
        } | Round {
            you: Paper,
            other: Rock
        } | Round {
            you: Scissors,
            other: Paper
        }
    ) {
        Win
    } else {
        Loss
    }
}

fn get_score_for_round(round: &Round) -> usize {
    let score_for_shape = match round.you {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    };

    let score_for_outcome = match get_round_outcome(round) {
        Win => 6,
        Draw => 3,
        Loss => 0,
    };

    score_for_shape + score_for_outcome
}

impl Solution for Day2 {
    const NAME: &'static str = "day2";
    type TInput = Day2Input;
    type TPt1Output = usize;
    type TPt2Output = ();

    fn parse_input(input_str: &str) -> Result<Self::TInput> {
        split_parse(input_str, "\n")
    }

    fn solve_pt1(input: &Self::TInput) -> Result<Self::TPt1Output> {
        Ok(input.iter().map(get_score_for_round).sum())
    }

    fn solve_pt2(_input: &Self::TInput) -> Result<Self::TPt2Output> {
        bail!("not impl");
    }
}

//fn get_round_outcome(you: Shape, opponent: Shape) -> usize {}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::util::get_input;

    use super::*;

    lazy_static! {
        static ref INPUT_TEST: Day2Input = get_input::<Day2>("test.txt").unwrap();
        static ref INPUT_MAIN: Day2Input = get_input::<Day2>("input.txt").unwrap();
    }

    #[test]
    fn test_pt1_result() {
        assert_eq!(10310, Day2::solve_pt1(&INPUT_MAIN).unwrap())
    }

    #[test]
    fn test_pt1() {
        assert_eq!(15, Day2::solve_pt1(&INPUT_TEST).unwrap())
    }

    #[test]
    fn test_parse_input() {
        let input: &Day2Input = &INPUT_TEST;
        assert_eq!(
            &vec![
                Round {
                    you: Paper,
                    other: Rock,
                },
                Round {
                    you: Rock,
                    other: Paper,
                },
                Round {
                    you: Scissors,
                    other: Scissors,
                }
            ],
            input
        );
    }
}
