use anyhow::{anyhow, Result};

use crate::{
    solution::{Solution, SolutionInput},
    util::split_parse,
};

type Day1Input = Vec<Vec<usize>>;

fn get_total_by_elf(input: &Day1Input) -> impl Iterator<Item = usize> + '_ {
    input.iter().map(|it| it.iter().sum::<usize>())
}

impl SolutionInput for Day1Input {
    fn parse(s: &str) -> Result<Self> {
        s.split("\n\n")
            .map(|it| split_parse(it, "\n"))
            .collect::<Result<_>>()
    }
}

pub struct Day1Pt1;

impl Solution for Day1Pt1 {
    const DAY: usize = 1;
    const PART: usize = 1;

    type TInput = Day1Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        get_total_by_elf(input)
            .max()
            .ok_or_else(|| anyhow!("empty input? {:?}", input))
    }
}

pub struct Day1Pt2;

impl Solution for Day1Pt2 {
    const DAY: usize = 1;
    const PART: usize = 2;

    type TInput = Day1Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut sums: Vec<_> = get_total_by_elf(input).collect();
        sums.sort();
        Ok(sums.iter().rev().take(3).sum())
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;

    use crate::util::get_input;

    use super::*;

    lazy_static! {
        static ref INPUT_TEST: Day1Input = get_input::<Day1Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day1Input = get_input::<Day1Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_input() {
        let sums: Vec<_> = get_total_by_elf(&INPUT_TEST).collect();
        assert_eq!(vec![6000, 4000, 11000, 24000, 10000], sums);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(24000, Day1Pt1::solve(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_pt1_result() {
        assert_eq!(74198, Day1Pt1::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt2() {
        assert_eq!(45000, Day1Pt2::solve(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_pt2_result() {
        assert_eq!(209914, Day1Pt2::solve(&INPUT_MAIN).unwrap());
    }
}
