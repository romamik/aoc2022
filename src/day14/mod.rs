mod map;
mod parser;

use self::parser::parse_lines;
use crate::{
    day14::map::MapPoint,
    solution::{Solution, SolutionInput},
};
use anyhow::Result;
use map::Map;

type Coord = i32;
type Point = (Coord, Coord);
type Line = Vec<Point>;

impl SolutionInput for Map {
    fn parse(input_str: &str) -> Result<Self> {
        let lines = parse_lines(input_str)?;
        let map = Map::create(&lines)?;
        Ok(map)
    }
}

fn add_sand(map: &mut Map, init_pos: Point) -> bool {
    fn next_pos(map: &Map, pos: &Point) -> Option<Point> {
        for d in [(0, 1), (-1, 1), (1, 1)] {
            let next_pos = (pos.0 + d.0, pos.1 + d.1);
            if map.at(&next_pos) == MapPoint::Empty {
                return Some(next_pos);
            }
        }
        None
    }

    let mut pos = init_pos;
    while pos.1 <= map.max_y {
        match next_pos(map, &pos) {
            Some(next_pos) => pos = next_pos,
            None => {
                map.set(&pos, MapPoint::Sand);
                return true;
            }
        }
    }
    false
}

pub struct Day14Pt1;
impl Solution for Day14Pt1 {
    const DAY: usize = 14;
    const PART: usize = 1;

    type TInput = Map;
    type TOutput = usize;

    fn solve(input: &Map) -> Result<Self::TOutput> {
        let mut map: Map = input.clone();
        let mut count = 0;
        while add_sand(&mut map, (500, 0)) {
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
        static ref INPUT_TEST: Map = get_input::<Day14Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Map = get_input::<Day14Pt1>("input.txt").unwrap();
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
        let mut map: Map = INPUT_TEST.clone();
        assert!(add_sand(&mut map, (500, 0)));
        assert_eq!(MapPoint::Sand, map.at(&(500, 8)));
        assert!(add_sand(&mut map, (500, 0)));
        assert_eq!(MapPoint::Sand, map.at(&(499, 8)));
        assert!(add_sand(&mut map, (500, 0)));
        assert_eq!(MapPoint::Sand, map.at(&(501, 8)));
        assert!(add_sand(&mut map, (500, 0)));
        assert_eq!(MapPoint::Sand, map.at(&(500, 7)));
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let map: &Map = &INPUT_TEST;
        assert_eq!(map.at(&(503, 4)), MapPoint::Wall);
        assert_eq!(map.at(&(502, 4)), MapPoint::Wall);
        assert_eq!(map.at(&(501, 4)), MapPoint::Empty);
        assert_eq!(map.max_y, 9);

        Ok(())
    }
}
