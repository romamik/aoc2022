use std::fmt::Debug;

use anyhow::Result;

pub trait Solution {
    type TInput: Debug;
    type TPt1Output: Debug;
    type TPt2Output: Debug;

    const NAME: &'static str;

    fn parse_input(input_str: &str) -> Result<Self::TInput>;
    fn solve_pt1(input: &Self::TInput) -> Result<Self::TPt1Output>;
    fn solve_pt2(input: &Self::TInput) -> Result<Self::TPt2Output>;
}
