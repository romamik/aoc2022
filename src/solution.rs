use std::fmt::Debug;

use anyhow::{Context, Result};

use crate::util::get_input;

pub trait SolutionInput: Debug + Sized {
    fn parse(input_str: &str) -> Result<Self>;
}

pub trait Solution {
    const DAY: usize;
    const PART: usize;

    type TInput: SolutionInput;
    type TOutput: Debug;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput>;

    fn print_result(result: &Self::TOutput) {
        print!("{:?}", result);
    }

    fn run() -> Result<()> {
        let input = get_input::<Self>("input.txt")?;

        let output =
            Self::solve(&input).context(format!("Day {}, Part {}", Self::DAY, Self::PART))?;
        print!("Day {} Part {} result: ", Self::DAY, Self::PART);
        Self::print_result(&output);
        println!();

        Ok(())
    }
}
