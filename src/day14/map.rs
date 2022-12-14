use std::collections::HashMap;

use anyhow::{ensure, Result};
use itertools::Itertools;

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

#[derive(Debug, Clone)]
pub struct Map {
    map: HashMap<Point, MapPoint>,
    pub max_y: Coord,
}

impl Map {
    pub fn create(lines: &[Line]) -> Result<Map> {
        let mut max_y = 0;
        let mut map = HashMap::new();
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
                    map.insert(pt, MapPoint::Wall);
                    max_y = max_y.max(pt.1);
                    if pt == (x1, y1) {
                        break;
                    }
                    pt.0 += dx;
                    pt.1 += dy;
                }
            }
        }

        Ok(Map { map, max_y })
    }

    pub fn at(&self, pt: &Point) -> MapPoint {
        self.map.get(pt).cloned().unwrap_or_default()
    }

    pub fn set(&mut self, pt: &Point, v: MapPoint) {
        self.map.insert(*pt, v);
    }
}
