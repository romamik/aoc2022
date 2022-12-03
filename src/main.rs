mod day1;
mod day2;
mod day3;
mod solution;
mod util;

use crate::solution::Solution;
use anyhow::Result;
use day1::{Day1Pt1, Day1Pt2};
use day2::{Day2Pt1, Day2Pt2};
use day3::{Day3Pt1, Day3Pt2};

fn main() -> Result<()> {
    Day1Pt1::run()?;
    Day1Pt2::run()?;
    Day2Pt1::run()?;
    Day2Pt2::run()?;
    Day3Pt1::run()?;
    Day3Pt2::run()?;
    Ok(())
}
