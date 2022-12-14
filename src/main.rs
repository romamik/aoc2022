mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod solution;
mod util;

use crate::solution::Solution;
use anyhow::Result;
use day1::{Day1Pt1, Day1Pt2};
use day10::{Day10Pt1, Day10Pt2};
use day11::{Day11Pt1, Day11Pt2};
use day12::{Day12Pt1, Day12Pt2};
use day13::{Day13Pt1, Day13Pt2};
use day14::{Day14Pt1, Day14Pt2};
use day15::{Day15Pt1, Day15Pt2};
use day16::Day16Pt1;
use day2::{Day2Pt1, Day2Pt2};
use day3::{Day3Pt1, Day3Pt2};
use day4::{Day4Pt1, Day4Pt2};
use day5::{Day5Pt1, Day5Pt2};
use day6::{Day6Pt1, Day6Pt2};
use day7::{Day7Pt1, Day7Pt2};
use day8::{Day8Pt1, Day8Pt2};
use day9::{Day9Pt1, Day9Pt2};

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
    Day9Pt1::run()?;
    Day9Pt2::run()?;
    Day10Pt1::run()?;
    Day10Pt2::run()?;
    Day11Pt1::run()?;
    Day11Pt2::run()?;
    Day12Pt1::run()?;
    Day12Pt2::run()?;
    Day13Pt1::run()?;
    Day13Pt2::run()?;
    Day14Pt1::run()?;
    Day14Pt2::run()?;
    Day15Pt1::run()?;
    Day15Pt2::run()?;
    Day16Pt1::run()?;
    Ok(())
}
