use self::stack_set::StackSet;
use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, bail, ensure, Context, Result};
use itertools::Itertools;

mod stack_set;

type Day5Input = (StackSet, Vec<(usize, char, char)>);

impl SolutionInput for Day5Input {
    fn parse(input_str: &str) -> Result<Self> {
        let (stack_set_input, cmd_input) = input_str
            .split("\n\n")
            .collect_tuple()
            .ok_or_else(|| anyhow!("bad number of sections in input"))?;
        let stack_set = stack_set_input.parse::<StackSet>()?;
        let commands = cmd_input
            .split('\n')
            .map(|cmd_line| {
                let elements: [&str; 6] = cmd_line
                    .split(' ')
                    .collect::<Vec<_>>()
                    .try_into()
                    .map_err(|v| anyhow!("cmd_line has too much items {:?}", v))?;
                match elements {
                    ["move", count_str, "from", from_str, "to", to_str] => {
                        ensure!(from_str.len() == 1, "bad from_str {:?}", from_str);
                        ensure!(to_str.len() == 1, "bad to_str {:?}", to_str);
                        let from = from_str
                            .chars()
                            .next()
                            .ok_or_else(|| anyhow!("bad from_str {:?}", from_str))?;
                        let to = to_str
                            .chars()
                            .next()
                            .ok_or_else(|| anyhow!("bad to_str {:?}", to_str))?;
                        let count = count_str
                            .parse::<usize>()
                            .context(format!("bad count_str {:?}", count_str))?;
                        Ok((count, from, to))
                    }
                    _ => {
                        bail!("bad cmd line format {:?}", cmd_line);
                    }
                }
            })
            .collect::<Result<Vec<_>>>()?;
        Ok((stack_set, commands))
    }
}

pub struct Day5Pt1;

impl Solution for Day5Pt1 {
    const DAY: usize = 5;
    const PART: usize = 1;

    type TInput = Day5Input;
    type TOutput = String;

    fn solve(input: &Day5Input) -> Result<String> {
        let mut stack_set = input.0.clone();
        for cmd in input.1.iter() {
            stack_set.apply_moves_by_1(cmd.0, cmd.1, cmd.2)?;
        }
        Ok(stack_set.get_top_names().iter().join(""))
    }
}

pub struct Day5Pt2;

impl Solution for Day5Pt2 {
    const DAY: usize = 5;
    const PART: usize = 2;

    type TInput = Day5Input;
    type TOutput = String;

    fn solve(input: &Day5Input) -> Result<String> {
        let mut stack_set = input.0.clone();
        for cmd in input.1.iter() {
            stack_set.apply_moves_by_chunks(cmd.0, cmd.1, cmd.2)?;
        }
        Ok(stack_set.get_top_names().iter().join(""))
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Day5Input = get_input::<Day5Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day5Input = get_input::<Day5Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_pt2_result() {
        assert_eq!("VHJDDCWRD", &Day5Pt2::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt2() {
        assert_eq!("MCD", &Day5Pt2::solve(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_pt1_result() {
        assert_eq!("JDTMRWCQJ", &Day5Pt1::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt1() {
        assert_eq!("CMZ", &Day5Pt1::solve(&INPUT_TEST).unwrap());
    }
}
