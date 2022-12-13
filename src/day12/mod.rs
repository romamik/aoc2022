use std::collections::{HashSet, VecDeque};

use anyhow::{anyhow, bail, ensure, Result};

use crate::{
    solution::{Solution, SolutionInput},
    util::Vec2d,
};

type Point = (isize, isize);

type Map = Vec2d<i8>;

impl Map {
    fn at(&self, pt: Point) -> Option<i8> {
        self.get(pt.0, pt.1).cloned()
    }

    fn find_path<EndPred, PathPred>(
        &self,
        start: Point,
        end_pred: EndPred,
        path_pred: PathPred,
    ) -> Result<usize>
    where
        EndPred: Fn(Point) -> bool,
        PathPred: Fn(i8, i8) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let start_height = self.at(start).ok_or_else(|| anyhow!("start out of map"))?;
        queue.push_back((start, start_height, 0));
        visited.insert(start);

        while let Some((pt, height, path_len)) = queue.pop_front() {
            if end_pred(pt) {
                return Ok(path_len);
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let next_pt = (pt.0 + dx, pt.1 + dy);

                if let Some(next_pt_height) = self.at(next_pt) {
                    if path_pred(height, next_pt_height) && !visited.contains(&next_pt) {
                        queue.push_back((next_pt, next_pt_height, path_len + 1));
                        visited.insert(next_pt);
                    }
                }
            }
        }

        bail!("path not found");
    }
}

#[derive(Debug)]
pub struct Input {
    map: Map,
    start: Point,
    end: Point,
}

impl SolutionInput for Input {
    fn parse(input_str: &str) -> Result<Self> {
        let mut start = None;
        let mut end = None;
        let map = Map::parse(input_str, |x, y, c| match c {
            c @ b'a'..=b'z' => Ok((c - b'a') as i8),
            b'S' => {
                ensure!(start.is_none());
                start = Some((x as isize, y as isize));
                Ok(0)
            }
            b'E' => {
                ensure!(end.is_none());
                end = Some((x as isize, y as isize));
                Ok((b'z' - b'a') as i8)
            }
            _ => bail!("unexpected char {:?}", c as char),
        })?;

        let start = start.ok_or_else(|| anyhow!("No start"))?;
        let end = end.ok_or_else(|| anyhow!("No end"))?;

        Ok(Input { map, start, end })
    }
}

pub struct Day12Pt1;

impl Solution for Day12Pt1 {
    const DAY: usize = 12;
    const PART: usize = 1;

    type TInput = Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        input.map.find_path(
            input.start,
            |pt| pt == input.end,
            |height, next_height| next_height <= height + 1,
        )
    }
}

pub struct Day12Pt2;

impl Solution for Day12Pt2 {
    const DAY: usize = 12;
    const PART: usize = 2;

    type TInput = Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        input.map.find_path(
            input.end,
            |pt| input.map.at(pt) == Some(0),
            |height, next_height| height <= next_height + 1,
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Input = get_input::<Day12Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Input = get_input::<Day12Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(386, Day12Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(29, Day12Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(391, Day12Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(31, Day12Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }
}
