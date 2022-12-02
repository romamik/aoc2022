use anyhow::{Context, Result};
use std::{fs, str::FromStr};

use crate::solution::Solution;

pub fn get_input<T: Solution>(name: &str) -> Result<T::TInput> {
    let input_str = read_file::<T>(name)?;
    T::parse_input(&input_str)
}

fn read_file<T: Solution>(name: &str) -> Result<String> {
    let full_name = format!("src/{}/{}", T::NAME, name);
    fs::read_to_string(&full_name).context(format!("reading {}", full_name))
}

pub fn split_parse<T>(data: &str, sep: &str) -> Result<Vec<T>>
where
    T: FromStr,
    Result<T, <T as FromStr>::Err>: Context<T, <T as FromStr>::Err>,
{
    data.split(sep)
        .map(|it| it.parse::<T>().context(format!("can't parse '{}'", it)))
        .collect()
}
