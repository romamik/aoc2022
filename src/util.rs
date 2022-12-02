use anyhow::{Context, Result};
use std::{fs, str::FromStr};

use crate::solution::{Solution, SolutionInput};

pub fn get_input<T: Solution + ?Sized>(name: &str) -> Result<T::TInput> {
    let file_name = format!("src/Day{}/{}", T::DAY, name);
    let input_str = fs::read_to_string(&file_name).context(format!("reading {:?}", &file_name))?;
    T::TInput::parse(&input_str).context(format!("parsing {:?}", &file_name))
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
