use anyhow::{anyhow, Result};

use crate::{solution::Solution, util::split_parse};

pub struct Day1;

type Day1Input = Vec<Vec<usize>>;

fn get_total_by_elf(input: &Day1Input) -> impl Iterator<Item = usize> + '_ {
    input.iter().map(|it| it.iter().sum::<usize>())
}

impl Solution for Day1 {
    type TInput = Day1Input;
    type TPt1Output = usize;
    type TPt2Output = usize;
    const NAME: &'static str = "day1";

    fn parse_input(input_str: &str) -> Result<Self::TInput> {
        input_str
            .split("\n\n")
            .map(|it| split_parse(it, "\n"))
            .collect::<Result<_>>()
    }

    fn solve_pt1(input: &Self::TInput) -> Result<Self::TPt1Output> {
        get_total_by_elf(input)
            .max()
            .ok_or_else(|| anyhow!("empty input? {:?}", input))
    }

    fn solve_pt2(input: &Self::TInput) -> Result<Self::TPt2Output> {
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
        static ref INPUT_TEST: Day1Input = get_input::<Day1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day1Input = get_input::<Day1>("input.txt").unwrap();
    }

    #[test]
    fn test_input() {
        let sums: Vec<_> = get_total_by_elf(&INPUT_TEST).collect();
        assert_eq!(vec![6000, 4000, 11000, 24000, 10000], sums);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(24000, Day1::solve_pt1(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_pt1_result() {
        assert_eq!(74198, Day1::solve_pt1(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt2() {
        assert_eq!(45000, Day1::solve_pt2(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_pt2_result() {
        assert_eq!(209914, Day1::solve_pt2(&INPUT_MAIN).unwrap());
    }
}
