use crate::{
    solution::{Solution, SolutionInput},
    util::Vec2d,
};
use anyhow::{anyhow, Result};

type Day8Input = Vec2d<isize>;

impl Day8Input {
    pub fn at(&self, x: isize, y: isize) -> isize {
        self.get(x, y).cloned().unwrap_or(-1)
    }

    pub fn size_x(&self) -> isize {
        self.size_x as isize
    }

    pub fn size_y(&self) -> isize {
        self.size_y as isize
    }

    pub fn count_visible_trees(&self) -> usize {
        fn look_from(
            map: &Day8Input,
            start: (isize, isize),
            end: (isize, isize),
            visible_trees: &mut [Vec<u8>],
        ) {
            let dx = (end.0 - start.0).signum();
            let dy = (end.1 - start.1).signum();
            assert!(dx == 0 || dy == 0);

            let mut x = start.0;
            let mut y = start.1;
            let mut prev_height = map.at(x - dx, y - dy);
            while x != end.0 || y != end.1 {
                let height = map.at(x, y);
                let visible = height > prev_height;
                if visible {
                    visible_trees[y as usize][x as usize] += 1;
                    prev_height = height;
                }
                x += dx;
                y += dy;
            }
        }

        let mut visible_trees: Vec<Vec<u8>> =
            vec![vec![0; self.size_x as usize]; self.size_y as usize];

        for x in 0..self.size_x() {
            look_from(self, (x, 0), (x, self.size_y() - 1), &mut visible_trees);
            look_from(self, (x, self.size_y() - 1), (x, 0), &mut visible_trees);
        }

        for y in 0..self.size_y() {
            look_from(self, (0, y), (self.size_x() - 1, y), &mut visible_trees);
            look_from(self, (self.size_x() - 1, y), (0, y), &mut visible_trees);
        }

        visible_trees
            .into_iter()
            .map(|line| line.into_iter().filter(|&v| v > 0).count())
            .sum()
    }

    pub fn viewing_distance(&self, start: (isize, isize), dir: (isize, isize)) -> usize {
        let mut x = start.0;
        let mut y = start.1;
        let start_height = self.at(x, y);
        let mut distance = 0;
        loop {
            x += dir.0;
            y += dir.1;

            let height = self.at(x, y);
            if height < 0 {
                break;
            }

            distance += 1;

            if height >= start_height {
                break;
            }
        }
        distance
    }
    pub fn scenic_score_at(&self, x: isize, y: isize) -> usize {
        let up = self.viewing_distance((x, y), (0, -1));
        let down = self.viewing_distance((x, y), (0, 1));
        let right = self.viewing_distance((x, y), (1, 0));
        let left = self.viewing_distance((x, y), (-1, 0));
        up * down * right * left
    }

    pub fn find_best_scenic_score(&self) -> Result<(usize, (isize, isize))> {
        let mut best = None;
        for y in 0..self.size_y() {
            for x in 0..self.size_x() {
                let score = self.scenic_score_at(x, y);
                match best {
                    None => best = Some((score, (x, y))),
                    Some((prev_score, _)) if prev_score < score => best = Some((score, (x, y))),
                    _ => (),
                }
            }
        }
        best.ok_or_else(|| anyhow!("not found"))
    }
}

impl SolutionInput for Day8Input {
    fn parse(input_str: &str) -> Result<Self> {
        Day8Input::parse(input_str, |_x, _y, c| -> Result<_> {
            Ok((c - b'0') as isize)
        })
    }
}

pub struct Day8Pt1;

impl Solution for Day8Pt1 {
    const DAY: usize = 8;
    const PART: usize = 1;

    type TInput = Day8Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input.count_visible_trees())
    }
}

pub struct Day8Pt2;

impl Solution for Day8Pt2 {
    const DAY: usize = 8;
    const PART: usize = 2;

    type TInput = Day8Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input.find_best_scenic_score()?.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Day8Input = get_input::<Day8Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day8Input = get_input::<Day8Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_scenic_score() -> Result<()> {
        let map: &Day8Input = &INPUT_TEST;

        assert_eq!(4, map.scenic_score_at(2, 1));
        assert_eq!(8, map.scenic_score_at(2, 3));
        Ok(())
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(368368, Day8Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(8, Day8Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_viewing_distance() -> Result<()> {
        let map: &Day8Input = &INPUT_TEST;

        assert_eq!(1, map.viewing_distance((2, 1), (0, -1)));
        assert_eq!(1, map.viewing_distance((2, 1), (-1, 0)));
        assert_eq!(2, map.viewing_distance((2, 1), (1, 0)));
        assert_eq!(2, map.viewing_distance((2, 1), (0, 1)));
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(1818, Day8Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(21, Day8Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let map: &Day8Input = &INPUT_TEST;
        assert_eq!(5, map.size_x);
        assert_eq!(5, map.size_y);
        assert_eq!(3, map.at(0, 0));
        assert_eq!(5, map.at(1, 1));
        assert_eq!(0, map.at(4, 4));
        assert_eq!(3, map.at(2, 4));
        assert_eq!(2, map.at(4, 2));
        Ok(())
    }
}
