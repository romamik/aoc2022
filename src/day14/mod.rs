mod map;
mod parser;

use self::parser::parse_lines;
use crate::{
    day14::map::MapPoint,
    solution::{Solution, SolutionInput},
};
use anyhow::{anyhow, Context, Result};
use map::Map;

type Coord = i32;
type Point = (Coord, Coord);
type Line = Vec<Point>;

impl SolutionInput for Vec<Line> {
    fn parse(input_str: &str) -> Result<Self> {
        parse_lines(input_str)
    }
}

fn add_sand(map: &mut Map, spawn_pos: Point) -> Result<bool> {
    fn next_pos(map: &Map, pos: &Point) -> Result<Option<Point>> {
        for d in [(0, 1), (-1, 1), (1, 1)] {
            let next_pos = (pos.0 + d.0, pos.1 + d.1);
            if map
                .at(&next_pos)
                .with_context(|| anyhow!("next_pos for {:?}", pos))?
                == MapPoint::Empty
            {
                return Ok(Some(next_pos));
            }
        }
        Ok(None)
    }

    if map.at(&spawn_pos)? != MapPoint::Empty {
        return Ok(false);
    }

    let mut pos = spawn_pos;
    let max_y = map.max.1;
    while pos.1 < max_y {
        match next_pos(map, &pos)? {
            Some(next_pos) => pos = next_pos,
            None => {
                map.set(&pos, MapPoint::Sand)?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub struct Day14Pt1;
impl Solution for Day14Pt1 {
    const DAY: usize = 14;
    const PART: usize = 1;

    type TInput = Vec<Line>;
    type TOutput = usize;

    fn solve(lines: &Vec<Line>) -> Result<Self::TOutput> {
        let spawn_pos = (500, 0);
        let mut map: Map = Map::create(lines, &spawn_pos, None)?;
        let mut count = 0;
        while add_sand(&mut map, spawn_pos)? {
            count += 1;
        }
        Ok(count)
    }
}

pub struct Day14Pt2;
impl Solution for Day14Pt2 {
    const DAY: usize = 14;
    const PART: usize = 2;

    type TInput = Vec<Line>;
    type TOutput = usize;

    fn solve(lines: &Vec<Line>) -> Result<Self::TOutput> {
        let spawn_pos = (500, 0);
        let mut map: Map = Map::create(lines, &spawn_pos, Some(2))?;
        let mut count = 0;
        while add_sand(&mut map, (500, 0))? {
            count += 1;
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{day14::map::MapPoint, util::get_input};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<Line> = get_input::<Day14Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Line> = get_input::<Day14Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(23416, Day14Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(93, Day14Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(817, Day14Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(24, Day14Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_add_sand() -> Result<()> {
        let mut map: Map = Map::create(&INPUT_TEST, &(500, 0), None)?;
        assert!(add_sand(&mut map, (500, 0))?);
        assert_eq!(MapPoint::Sand, map.at(&(500, 8))?);
        assert!(add_sand(&mut map, (500, 0))?);
        assert_eq!(MapPoint::Sand, map.at(&(499, 8))?);
        assert!(add_sand(&mut map, (500, 0))?);
        assert_eq!(MapPoint::Sand, map.at(&(501, 8))?);
        assert!(add_sand(&mut map, (500, 0))?);
        assert_eq!(MapPoint::Sand, map.at(&(500, 7))?);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let map: Map = Map::create(&INPUT_TEST, &(500, 0), None)?;
        assert_eq!(map.at(&(503, 4))?, MapPoint::Wall);
        assert_eq!(map.at(&(502, 4))?, MapPoint::Wall);
        assert_eq!(map.at(&(501, 4))?, MapPoint::Empty);
        assert_eq!(map.max.1, 9);

        Ok(())
    }
}
