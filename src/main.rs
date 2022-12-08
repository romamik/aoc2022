mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod solution;
mod util;

use crate::solution::Solution;
use anyhow::Result;
use day1::{Day1Pt1, Day1Pt2};
use day2::{Day2Pt1, Day2Pt2};
use day3::{Day3Pt1, Day3Pt2};
use day4::{Day4Pt1, Day4Pt2};
use day5::{Day5Pt1, Day5Pt2};
use day6::{Day6Pt1, Day6Pt2};
use day7::{Day7Pt1, Day7Pt2};
use day8::{Day8Pt1, Day8Pt2};

fn main() -> Result<()> {
    Day1Pt1::run()?;
    Day1Pt2::run()?;
    Day2Pt1::run()?;
    Day2Pt2::run()?;
    Day3Pt1::run()?;
    Day3Pt2::run()?;
    Day4Pt1::run()?;
    Day4Pt2::run()?;
    Day5Pt1::run()?;
    Day5Pt2::run()?;
    Day6Pt1::run()?;
    Day6Pt2::run()?;
    Day7Pt1::run()?;
    Day7Pt2::run()?;
    Day8Pt1::run()?;
    Day8Pt2::run()?;
    Ok(())
}
