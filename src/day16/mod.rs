use std::collections::HashMap;

use anyhow::Result;

use crate::solution::{Solution, SolutionInput};

use self::parser::parse_system;

mod parser;

#[derive(Debug, PartialEq, Eq)]
pub struct Room {
    flow: usize,
    tunnels: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct System {
    starting_room: String,
    rooms: HashMap<String, Room>,
}

impl SolutionInput for System {
    fn parse(input_str: &str) -> Result<Self> {
        parse_system(input_str)
    }
}

pub struct Day16Pt1;
impl Solution for Day16Pt1 {
    const DAY: usize = 16;
    const PART: usize = 1;

    type TInput = System;
    type TOutput = usize;

    fn solve(_input: &Self::TInput) -> Result<Self::TOutput> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: System = get_input::<Day16Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: System = get_input::<Day16Pt1>("input.txt").unwrap();
    }
}
