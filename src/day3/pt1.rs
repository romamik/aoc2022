use super::sack_parts::SackParts;
use crate::solution::Solution;
use crate::solution::SolutionInput;
use crate::util::split_parse;
use anyhow::Result;

pub type Day3Pt1Input = Vec<SackParts>;

impl SolutionInput for Day3Pt1Input {
    fn parse(input_str: &str) -> Result<Self> {
        split_parse(input_str, "\n")
    }
}

pub struct Day3Pt1;

impl Solution for Day3Pt1 {
    const DAY: usize = 3;
    const PART: usize = 1;

    type TInput = Day3Pt1Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input
            .iter()
            .map(|sack| -> Result<usize> { Ok(sack.find_duplicate_item()?.priority()) })
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Day3Pt1Input = get_input::<Day3Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day3Pt1Input = get_input::<Day3Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_day3_pt1() {
        assert_eq!(157, Day3Pt1::solve(&INPUT_TEST).unwrap());
    }
}
