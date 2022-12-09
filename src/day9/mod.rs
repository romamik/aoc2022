use crate::{
    solution::{Solution, SolutionInput},
    util::split_parse,
};
use anyhow::{anyhow, bail, Context, Error, Result};
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};
use Direction::*;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
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

trait PointMover {
    fn move_pt(&self, pt: &mut (i32, i32));
}

impl PointMover for (i32, i32) {
    fn move_pt(&self, pt: &mut (i32, i32)) {
        pt.0 += self.0;
        pt.1 += self.1;
    }
}

impl PointMover for Direction {
    fn move_pt(&self, pt: &mut (i32, i32)) {
        self.delta().move_pt(pt)
    }
}

pub type Day9Input = Vec<MoveCommand>;

impl SolutionInput for Day9Input {
    fn parse(input_str: &str) -> Result<Self> {
        split_parse(input_str, "\n")
    }
}

fn get_tail_move(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x = head.0 - tail.0;
    let y = head.1 - tail.1;

    if x.abs() <= 1 && y.abs() <= 1 {
        (0, 0)
    } else {
        (x.signum(), y.signum())
    }
}

fn _print_state(head: (i32, i32), tail: (i32, i32)) {
    let x0 = [head.0, tail.0, 0].into_iter().min().unwrap();
    let y0 = [head.1, tail.1, 0].into_iter().min().unwrap();
    let x1 = [head.0, tail.0, 0].into_iter().max().unwrap();
    let y1 = [head.1, tail.1, 0].into_iter().max().unwrap();
    for y in y0..=y1 {
        for x in x0..=x1 {
            let is_head = x == head.0 && y == head.1;
            let is_tail = x == tail.0 && y == tail.1;
            let is_start = x == 0 && y == 0;
            let c = if is_head {
                'H'
            } else if is_tail {
                'T'
            } else if is_start {
                's'
            } else {
                '.'
            };
            print!("{}", c)
        }
        println!()
    }
}

pub struct Day9Pt1;

impl Solution for Day9Pt1 {
    const DAY: usize = 9;
    const PART: usize = 1;

    type TInput = Day9Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);
        visited.insert(tail);
        for command in input.iter() {
            for _ in 0..command.count {
                command.dir.move_pt(&mut head);
                let tail_move = get_tail_move(head, tail);
                tail_move.move_pt(&mut tail);
                visited.insert(tail);
            }
        }
        Ok(visited.len())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Day9Input = get_input::<Day9Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day9Input = get_input::<Day9Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(5779, Day9Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(13, Day9Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let commands: &Day9Input = &INPUT_TEST;
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
