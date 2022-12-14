use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;

use crate::util::Vec2d;

use super::{Coord, Line, Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapPoint {
    Empty,
    Wall,
    Sand,
}

impl Default for MapPoint {
    fn default() -> Self {
        MapPoint::Empty
    }
}

pub type Map = Vec2d<MapPoint, Coord>;

impl Map {
    pub fn create(lines: &[Line], spawn_pos: &Point, floor_offset: Option<Coord>) -> Result<Map> {
        fn include_pt(min: &mut Point, max: &mut Point, &(x, y): &Point) {
            min.0 = min.0.min(x);
            max.0 = max.0.max(x);
            min.1 = min.1.min(y);
            max.1 = max.1.max(y);
        }

        let mut min = *spawn_pos;
        let mut max = *spawn_pos;

        lines
            .iter()
            .flat_map(|line| line.iter())
            .for_each(|pt| include_pt(&mut min, &mut max, pt));

        if let Some(off) = floor_offset {
            max.1 += off;
        }

        // sand cannot go more horizonally then verically
        let max_fall_height = max.1 - spawn_pos.1;
        include_pt(
            &mut min,
            &mut max,
            &(spawn_pos.0 - max_fall_height, spawn_pos.1 + max_fall_height),
        );
        include_pt(
            &mut min,
            &mut max,
            &(spawn_pos.0 + max_fall_height, spawn_pos.1 + max_fall_height),
        );

        let mut map = Vec2d::new(min, max, MapPoint::Empty)?;

        for line in lines {
            for (&(x0, y0), &(x1, y1)) in line.iter().tuple_windows::<(_, _)>() {
                let dx = (x1 - x0).signum();
                let dy = (y1 - y0).signum();
                ensure!(
                    (dx == 0 || dy == 0) && dx + dy != 0,
                    "must be horizontal or vertical {:?}",
                    (x0, y0, x1, y1)
                );
                let mut pt = (x0, y0);
                loop {
                    map.set(&pt, MapPoint::Wall)?;
                    if pt == (x1, y1) {
                        break;
                    }
                    pt.0 += dx;
                    pt.1 += dy;
                }
            }
        }

        if floor_offset.is_some() {
            for x in min.0..=max.0 {
                map.set(&(x, max.1), MapPoint::Wall)?;
            }
        }

        Ok(map)
    }

    pub fn at(&self, pt: &Point) -> Result<MapPoint> {
        self.get(pt)
            .cloned()
            .ok_or_else(|| anyhow!("out of map {:?}, {:?}-{:?}", pt, self.min, self.max))
    }

    pub fn set(&mut self, pt: &Point, v: MapPoint) -> Result<()> {
        let min = self.min;
        let max = self.max;
        let map_pt = self
            .get_mut(pt)
            .ok_or_else(|| anyhow!("out of map {:?}, {:?}-{:?}", pt, min, max))?;
        *map_pt = v;

        Ok(())
    }
}
