use anyhow::{anyhow, ensure, Context, Result};
use num_traits::{FromPrimitive, Num, ToPrimitive};
use std::{fmt::Debug, fs, str::FromStr};

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
pub struct Vec2d<ItemT, CoordT> {
    vec: Vec<ItemT>,
    pub min: (CoordT, CoordT),
    pub max: (CoordT, CoordT),
    pub size_x: CoordT,
    pub size_y: CoordT,
}

impl<ItemT: Copy, CoordT: Debug + Copy + Num + PartialOrd + ToPrimitive + FromPrimitive>
    Vec2d<ItemT, CoordT>
{
    pub fn new(min: (CoordT, CoordT), max: (CoordT, CoordT), init_val: ItemT) -> Result<Self> {
        let size_x = CoordT::one() + max.0 - min.0;
        let size_y = CoordT::one() + max.1 - min.1;
        let vec_size = (size_x * size_y)
            .to_usize()
            .ok_or_else(|| anyhow!("failed convert vec_size to usize"))?;
        let vec = vec![init_val; vec_size];
        Ok(Vec2d {
            vec,
            min,
            max,
            size_x,
            size_y,
        })
    }

    pub fn parse<ElemPredE, ElemPred>(
        input_str: &str,
        min: (CoordT, CoordT),
        mut elem_pred: ElemPred,
    ) -> Result<Vec2d<ItemT, CoordT>>
    where
        Result<ItemT, ElemPredE>: Context<ItemT, ElemPredE>,
        ElemPred: FnMut(usize, usize, u8) -> Result<ItemT, ElemPredE>,
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

        let max = (
            min.0 + CoordT::from_usize(size_x).unwrap() - CoordT::one(),
            min.1 + CoordT::from_usize(size_y).unwrap() - CoordT::one(),
        );

        Ok(Vec2d {
            vec,
            min,
            max,
            size_x: CoordT::from_usize(size_x).unwrap(),
            size_y: CoordT::from_usize(size_y).unwrap(),
        })
    }

    pub fn get(&self, pt: &(CoordT, CoordT)) -> Option<&ItemT> {
        if self.is_inside(pt) {
            Some(&self.vec[self.offset(pt)])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, pt: &(CoordT, CoordT)) -> Option<&mut ItemT> {
        if self.is_inside(pt) {
            let offset = self.offset(pt);
            Some(&mut self.vec[offset])
        } else {
            None
        }
    }

    pub fn is_inside(&self, pt: &(CoordT, CoordT)) -> bool {
        pt.0 >= self.min.0 && pt.0 <= self.max.0 && pt.1 >= self.min.1 && pt.1 <= self.max.1
    }

    fn offset(&self, pt: &(CoordT, CoordT)) -> usize {
        // not that I like these unwraps...
        ((pt.0 - self.min.0) + (pt.1 - self.min.1) * self.size_x)
            .to_usize()
            .unwrap()
    }
}
