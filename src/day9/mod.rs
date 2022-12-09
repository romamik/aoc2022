use crate::{
    solution::{Solution, SolutionInput},
    util::split_parse,
};
use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
use Direction::*;

type Point = (i32, i32);

trait Movable {
    fn move_by_point(&mut self, pt: &Point);

    fn move_by_dir(&mut self, dir: &Direction) {
        self.move_by_point(&dir.delta())
    }
}

impl Movable for Point {
    fn move_by_point(&mut self, pt: &Point) {
        self.0 += pt.0;
        self.1 += pt.1;
    }
}

type Rope = Vec<Point>;

fn get_tail_move(head: Point, tail: Point) -> Point {
    let x = head.0 - tail.0;
    let y = head.1 - tail.1;

    if x.abs() <= 1 && y.abs() <= 1 {
        (0, 0)
    } else {
        (x.signum(), y.signum())
    }
}

impl Movable for Rope {
    fn move_by_point(&mut self, pt: &Point) {
        if let Some(head) = self.first_mut() {
            head.move_by_point(pt)
        }
        for i in 1..self.len() {
            let mv = get_tail_move(self[i - 1], self[i]);
            self[i].move_by_point(&mv);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn delta(&self) -> Point {
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Direction> {
        let dir = match s {
            "U" => Up,
            "D" => Down,
            "R" => Right,
            "L" => Left,
            _ => bail!("unexpected direction {:?}", s),
        };
        Ok(dir)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MoveCommand {
    dir: Direction,
    count: usize,
}

impl FromStr for MoveCommand {
    type Err = Error;

    fn from_str(s: &str) -> Result<MoveCommand> {
        let (dir_str, count_str) = s
            .split(' ')
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| anyhow!("bad input {:?}", s))?;

        let dir = dir_str.parse().with_context(|| anyhow!("{:?}", dir_str))?;
        let count = count_str
            .parse()
            .with_context(|| anyhow!("{:?}", count_str))?;

        Ok(MoveCommand { dir, count })
    }
}

pub type CommandList = Vec<MoveCommand>;

impl SolutionInput for CommandList {
    fn parse(input_str: &str) -> Result<Self> {
        split_parse(input_str, "\n")
    }
}

fn get_unique_tail_positions_count(rope_len: usize, commands: &CommandList) -> usize {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut rope = vec![(0, 0); rope_len];
    visited.insert(*rope.last().unwrap());
    for command in commands.iter() {
        for _ in 0..command.count {
            rope.move_by_dir(&command.dir);
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

pub struct Day9Pt1;

impl Solution for Day9Pt1 {
    const DAY: usize = 9;
    const PART: usize = 1;

    type TInput = CommandList;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(get_unique_tail_positions_count(2, input))
    }
}

pub struct Day9Pt2;

impl Solution for Day9Pt2 {
    const DAY: usize = 9;
    const PART: usize = 2;

    type TInput = CommandList;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(get_unique_tail_positions_count(10, input))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST_1: CommandList = get_input::<Day9Pt1>("test.txt").unwrap();
        static ref INPUT_TEST_2: CommandList = get_input::<Day9Pt1>("test2.txt").unwrap();
        static ref INPUT_MAIN: CommandList = get_input::<Day9Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(2331, Day9Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(1, Day9Pt2::solve(&INPUT_TEST_1)?);
        assert_eq!(36, Day9Pt2::solve(&INPUT_TEST_2)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(5779, Day9Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(13, Day9Pt1::solve(&INPUT_TEST_1)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let commands: &CommandList = &INPUT_TEST_1;
        assert_eq!(
            &MoveCommand {
                dir: Right,
                count: 4
            },
            commands.first().unwrap(),
        );
        assert_eq!(
            &MoveCommand {
                dir: Right,
                count: 2
            },
            commands.last().unwrap(),
        );
        assert_eq!(8, commands.len());
        Ok(())
    }
}
