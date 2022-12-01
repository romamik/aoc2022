use anyhow::{Context, Result};

use crate::{day1::Day1, solution::Solution, util::get_input};

mod day1;
mod solution;
mod util;

fn run<T: Solution>() -> Result<()> {
    let input = get_input::<T>("input.txt")?;

    let pt1_output = T::solve_pt1(&input).context(format!("{} pt1 failed", T::NAME))?;
    println!("{} pt1: {:?}", T::NAME, pt1_output);

    let pt2_output = T::solve_pt2(&input).context(format!("{} pt2 failed", T::NAME))?;
    println!("{} pt2: {:?}", T::NAME, pt2_output);

    Ok(())
}

fn main() -> Result<()> {
    run::<Day1>()?;
    Ok(())
}
