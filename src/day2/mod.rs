use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use itertools::Itertools;

use crate::{
    solution::{Solution, SolutionInput},
    util::split_parse,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

type Day2Pt1Input = Vec<Round>;

impl SolutionInput for Day2Pt1Input {
    fn parse(input_str: &str) -> Result<Self> {
        split_parse(input_str, "\n")
    }
}

pub struct Day2Pt1;

impl Solution for Day2Pt1 {
    const DAY: usize = 2;
    const PART: usize = 1;

    type TInput = Day2Pt1Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input.iter().map(get_score_for_round).sum())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RoundPlan {
    other: Shape,
    outcome: RoundOutcome,
}

impl FromStr for RoundPlan {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (str0, str1) = s
            .split(' ')
            .collect_tuple()
            .ok_or_else(|| anyhow!("bad input string: {:?}", s))?;

        let shape = match str0 {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => bail!("unexpected string for shape: {:?}, {:?}", str0, s),
        };

        let outcome = match str1 {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => bail!("unexpected string for outcome: {:?}, {:?}", str1, s),
        };

        Ok(RoundPlan {
            other: shape,
            outcome,
        })
    }
}

fn choose_your_shape_for_plan(plan: &RoundPlan) -> Shape {
    match plan.outcome {
        Draw => plan.other,
        Win => match plan.other {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        Loss => match plan.other {
            Paper => Rock,
            Scissors => Paper,
            Rock => Scissors,
        },
    }
}

fn get_score_for_plan(plan: &RoundPlan) -> usize {
    let round = Round {
        you: choose_your_shape_for_plan(plan),
        other: plan.other,
    };
    get_score_for_round(&round)
}

type Day2Pt2Input = Vec<RoundPlan>;

impl SolutionInput for Day2Pt2Input {
    fn parse(input_str: &str) -> Result<Self> {
        split_parse(input_str, "\n")
    }
}

pub struct Day2Pt2;

impl Solution for Day2Pt2 {
    const DAY: usize = 2;
    const PART: usize = 2;

    type TInput = Day2Pt2Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input.iter().map(get_score_for_plan).sum())
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::util::get_input;

    use super::*;

    lazy_static! {
        static ref INPUT_TEST: Day2Pt1Input = get_input::<Day2Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day2Pt1Input = get_input::<Day2Pt1>("input.txt").unwrap();
        static ref INPUT_TEST_2: Day2Pt2Input = get_input::<Day2Pt2>("test.txt").unwrap();
        static ref INPUT_MAIN_2: Day2Pt2Input = get_input::<Day2Pt2>("input.txt").unwrap();
    }

    #[test]
    fn test_pt2_result() {
        assert_eq!(14859, Day2Pt2::solve(&INPUT_MAIN_2).unwrap())
    }

    #[test]
    fn test_pt2() {
        assert_eq!(12, Day2Pt2::solve(&INPUT_TEST_2).unwrap())
    }

    #[test]
    fn test_parse_input_2() {
        let input: &Day2Pt2Input = &INPUT_TEST_2;
        assert_eq!(
            &vec![
                RoundPlan {
                    other: Rock,
                    outcome: Draw,
                },
                RoundPlan {
                    other: Paper,
                    outcome: Loss
                },
                RoundPlan {
                    other: Scissors,
                    outcome: Win,
                }
            ],
            input
        );
    }

    #[test]
    fn test_pt1_result() {
        assert_eq!(10310, Day2Pt1::solve(&INPUT_MAIN).unwrap())
    }

    #[test]
    fn test_pt1() {
        assert_eq!(15, Day2Pt1::solve(&INPUT_TEST).unwrap())
    }

    #[test]
    fn test_parse_input() {
        let input: &Day2Pt1Input = &INPUT_TEST;
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
