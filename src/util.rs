use anyhow::{anyhow, ensure, Context, Result};
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

#[derive(Debug)]
pub struct Vec2d<T> {
    vec: Vec<T>,
    pub size_x: usize,
    pub size_y: usize,
}

impl<T> Vec2d<T> {
    pub fn parse<ElemPredE, ElemPred>(input_str: &str, mut elem_pred: ElemPred) -> Result<Vec2d<T>>
    where
        Result<T, ElemPredE>: Context<T, ElemPredE>,
        ElemPred: FnMut(usize, usize, u8) -> Result<T, ElemPredE>,
    {
        let mut size_x = None;
        let mut size_y = 0;
        let mut vec = Vec::new();
        for (y, line) in input_str.split('\n').enumerate() {
            let mut line_len = 0;

            for (x, c) in line.bytes().enumerate() {
                let elem = elem_pred(x, y, c)
                    .with_context(|| anyhow!("at line {}, pos {}", y + 1, x + 1))?;
                vec.push(elem);

                line_len += 1;
            }

            match size_x {
                None => size_x = Some(line_len),
                Some(size_x) => ensure!(size_x == line_len),
            }

            size_y += 1;
        }

        let size_x = size_x.ok_or_else(|| anyhow!("No input"))?;

        Ok(Vec2d {
            vec,
            size_x,
            size_y,
        })
    }

    pub fn get<X: TryInto<usize> + PartialOrd + Default>(&self, x: X, y: X) -> Option<&T> {
        if x >= X::default() && y >= X::default() {
            let x: usize = match x.try_into() {
                Ok(x) => x,
                Err(_) => return None,
            };
            let y: usize = match y.try_into() {
                Ok(y) => y,
                Err(_) => return None,
            };
            if x < self.size_x && y < self.size_y {
                return Some(&self.vec[x + y * self.size_x]);
            }
        }
        None
    }
}
