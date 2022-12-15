mod parser;

use self::parser::parse_sensors;
use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

type Coord = i64;
type Point = (Coord, Coord);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sensor {
    pos: Point,
    beacon: Point,
}

fn manhattan_distance(a: Point, b: Point) -> Coord {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl Sensor {
    pub fn get_restricted_x(&self, at_y: Coord) -> Option<(Coord, Coord)> {
        let (x, y) = self.pos;
        let beacon_dist = manhattan_distance(self.pos, self.beacon);
        let dy = (at_y - y).abs();
        let dx = beacon_dist - dy;
        if dx >= 0 {
            Some((x - dx, x + dx))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum PtType {
    // order is important, Start should be before End, with Beacon and Sensor between them
    Start,
    Beacon,
    Sensor,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt {
    typ: PtType,
    x: Coord,
}

impl Pt {
    fn new(typ: PtType, x: Coord) -> Pt {
        Pt { typ, x }
    }
}

impl PartialOrd for Pt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Pt {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;

        match self.x.cmp(&other.x) {
            cmp @ (Greater | Less) => cmp,
            Equal => self.typ.cmp(&other.typ),
        }
    }
}

#[derive(Debug)]
struct LineRestrictions {
    vec: Vec<Pt>, // sorted by coord
}

impl LineRestrictions {
    fn new(sensors: &[Sensor], at_y: Coord) -> LineRestrictions {
        use PtType::*;

        // points where beacon or sensor is situated
        let mut pts = HashSet::new();
        for sensor in sensors.iter() {
            for (typ, (x, y)) in [(Sensor, sensor.pos), (Beacon, sensor.beacon)] {
                if y == at_y {
                    pts.insert(Pt::new(typ, x));
                }
            }
        }

        let mut vec = sensors
            .iter()
            .filter_map(|sensor| sensor.get_restricted_x(at_y))
            .flat_map(|(start, end)| [Pt::new(Start, start), Pt::new(End, end)].into_iter())
            .chain(pts.into_iter())
            .collect_vec();

        vec.sort();

        LineRestrictions { vec }
    }

    fn count_restricted(&self) -> Coord {
        let mut result = 0;
        let mut start_count = 0;
        let mut start_x = 0;

        for pt in self.vec.iter() {
            match pt.typ {
                PtType::Start => {
                    if start_count == 0 {
                        start_x = pt.x;
                    }
                    start_count += 1;
                }
                PtType::Beacon | PtType::Sensor => {
                    if start_count > 0 {
                        result -= 1;
                    }
                }
                PtType::End => {
                    start_count -= 1;
                    if start_count == 0 {
                        let len = pt.x - start_x + 1;
                        result += len;
                    }
                }
            }
        }

        result
    }

    fn find_free(&self, min_x: Coord, max_x: Coord) -> Option<Coord> {
        // also there is the case when restrictions end before max_x by I decided not to take it into account
        let mut start_count = 0;

        for pt in self.vec.iter() {
            match pt.typ {
                PtType::Start => {
                    if start_count == 0 {
                        // we know that previous point (pt.x - 1) is not restricted
                        let empty_x = pt.x - 1;
                        if (min_x..=max_x).contains(&empty_x) {
                            return Some(empty_x);
                        }
                    }
                    start_count += 1;
                }
                PtType::End => {
                    start_count -= 1;
                }
                _ => (),
            }
        }
        None
    }
}

fn find_free(sensors: &[Sensor], min: Point, max: Point) -> Option<Point> {
    for y in min.1..=max.1 {
        let restrictions = LineRestrictions::new(sensors, y);
        if let Some(x) = restrictions.find_free(min.0, max.0) {
            return Some((x, y));
        }
    }
    None
}

impl SolutionInput for Vec<Sensor> {
    fn parse(input_str: &str) -> Result<Self> {
        parse_sensors(input_str)
    }
}

pub struct Day15Pt1;
impl Solution for Day15Pt1 {
    const DAY: usize = 15;
    const PART: usize = 1;

    type TInput = Vec<Sensor>;
    type TOutput = Coord;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let restriction = LineRestrictions::new(input, 2000000);
        Ok(restriction.count_restricted())
    }
}

pub struct Day15Pt2;
impl Solution for Day15Pt2 {
    const DAY: usize = 15;
    const PART: usize = 2;

    type TInput = Vec<Sensor>;
    type TOutput = Coord;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let pt = find_free(input, (0, 0), (4000000, 4000000))
            .ok_or_else(|| anyhow!("free space not found"))?;
        Ok(pt.0 * 4000000 + pt.1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use all_asserts::assert_lt;
    use lazy_static::lazy_static;
    use ntest::timeout;

    lazy_static! {
        static ref INPUT_TEST: Vec<Sensor> = get_input::<Day15Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Sensor> = get_input::<Day15Pt1>("input.txt").unwrap();
    }

    #[test]
    #[timeout(1000)]
    fn test_part2_result() -> Result<()> {
        assert_eq!(10229191267339, Day15Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(5144286, Day15Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_find_free() {
        assert_eq!(find_free(&INPUT_TEST, (0, 0), (20, 20)), Some((14, 11)));
    }

    #[test]
    fn test_line_restrictions() {
        /*
                         1    1    2    2
               0    5    0    5    0    5
         9 ...#########################...
        10 ..####B######################..
        11 .###S#############.###########.
        */
        let r_9 = LineRestrictions::new(&INPUT_TEST, 9);
        assert_eq!(r_9.count_restricted(), 25);
        assert_eq!(r_9.find_free(0, 20), None);

        let r_10 = LineRestrictions::new(&INPUT_TEST, 10);
        assert_eq!(r_10.count_restricted(), 26);
        assert_eq!(r_10.find_free(0, 20), None);

        let r_11 = LineRestrictions::new(&INPUT_TEST, 11);
        assert_eq!(r_11.count_restricted(), 27);
        assert_eq!(r_11.find_free(0, 20), Some(14));
    }

    #[test]
    fn test_pt_order() {
        assert_lt!(PtType::Start, PtType::Beacon);
        assert_lt!(PtType::Start, PtType::Sensor);
        assert_lt!(PtType::Start, PtType::End);
        assert_lt!(PtType::Beacon, PtType::End);
        assert_lt!(PtType::Sensor, PtType::End);
    }

    #[test]
    fn test_get_restricted_x() {
        /*
                       1    1    2    2
             0    5    0    5    0    5
        -2 ..........#.................
        -1 .........###................
         0 ....S...#####...............
         1 .......#######........S.....
         2 ......#########S............
         3 .....###########SB..........
         4 ....#############...........
         5 ...###############..........
         6 ..#################.........
         7 .#########S#######S#........
         8 ..#################.........
         9 ...###############..........
        10 ....B############...........
        11 ..S..###########............
        12 ......#########.............
        13 .......#######..............
        14 ........#####.S.......S.....
        15 B........###................
        16 ..........#SB...............
        17 ................S..........B
        */

        let sensor = Sensor {
            pos: (8, 7),
            beacon: (2, 10),
        };
        assert_eq!(sensor.get_restricted_x(7), Some((-1, 17)));
        assert_eq!(sensor.get_restricted_x(10), Some((2, 14)));
        assert_eq!(sensor.get_restricted_x(16), Some((8, 8)));
        assert_eq!(sensor.get_restricted_x(-2), Some((8, 8)));
        assert_eq!(sensor.get_restricted_x(17), None);
        assert_eq!(sensor.get_restricted_x(-3), None);
    }
}
