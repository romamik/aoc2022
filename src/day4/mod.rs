use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;

type Range = (usize, usize);
type RangePair = (Range, Range);
type Day4Input = Vec<RangePair>;

fn parse_range(s: &str) -> Result<Range> {
    let vec: Vec<_> = s
        .split('-')
        .map(|num_str| {
            num_str
                .parse::<usize>()
                .with_context(|| format!("{:?}", num_str))
        })
        .try_collect()?;
    let range = vec
        .iter()
        .cloned()
        .collect_tuple::<(_, _)>()
        .ok_or_else(|| anyhow!("wrong number of items {:?}", vec))?;
    Ok(range)
}

fn parse_range_pair(s: &str) -> Result<RangePair> {
    let vec: Vec<_> = s.split(',').map(parse_range).try_collect()?;
    let pair = vec
        .into_iter()
        .collect_tuple::<(_, _)>()
        .ok_or_else(|| anyhow!("wrong number of items"))?;
    Ok(pair)
}

fn range_contains_range(a: &Range, b: &Range) -> bool {
    let rng = a.0..=a.1;
    rng.contains(&b.0) && rng.contains(&b.1)
}

impl SolutionInput for Day4Input {
    fn parse(input_str: &str) -> Result<Self> {
        input_str.split('\n').map(parse_range_pair).collect()
    }
}

pub struct Day4Pt1;

impl Solution for Day4Pt1 {
    const DAY: usize = 4;
    const PART: usize = 1;

    type TInput = Day4Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input
            .iter()
            .filter(|pair| {
                range_contains_range(&pair.0, &pair.1) || range_contains_range(&pair.1, &pair.0)
            })
            .count())
    }
}

#[cfg(test)]
mod tests {

    use lazy_static::lazy_static;

    use crate::util::get_input;

    use super::*;
    lazy_static! {
        static ref INPUT_TEST: Day4Input = get_input::<Day4Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day4Input = get_input::<Day4Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_pt1_result() {
        assert_eq!(540, Day4Pt1::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt1() {
        assert_eq!(2, Day4Pt1::solve(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_parse() {
        assert_eq!((11, 12), parse_range("11-12").unwrap());
        assert_eq!(
            ((11, 12), (12, 13)),
            parse_range_pair("11-12,12-13").unwrap()
        );

        let input: &Day4Input = &INPUT_TEST;
        assert_eq!(
            &vec![
                ((2, 4), (6, 8)),
                ((2, 3), (4, 5)),
                ((5, 7), (7, 9)),
                ((2, 8), (3, 7)),
                ((6, 6), (4, 6)),
                ((2, 6), (4, 8)),
            ],
            input
        )
    }
}
