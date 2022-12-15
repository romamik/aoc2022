use anyhow::{anyhow, ensure, Context, Result};

use super::{Coord, Point, Sensor};

pub fn parse_sensor(s: &str) -> Result<Sensor> {
    let (rem, sensor) = nom_parser::parse_sensor(s).map_err(|e| e.to_owned())?;
    ensure!(rem.len() == 0);
    Ok(sensor)
}

pub fn parse_sensors(s: &str) -> Result<Vec<Sensor>> {
    s.split('\n')
        .map(|s| parse_sensor(s).with_context(|| anyhow!("parsing {:?}", s)))
        .collect()
}

mod nom_parser {

    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{char, digit1, one_of, space0},
        combinator::{map, map_res, opt, recognize},
        sequence::{delimited, pair, preceded, separated_pair},
        IResult,
    };

    fn parse_num(s: &str) -> IResult<&str, Coord> {
        map_res(recognize(pair(opt(one_of("+-")), digit1)), |s: &str| {
            s.parse()
        })(s)
    }

    fn parse_point(s: &str) -> IResult<&str, Point> {
        separated_pair(
            preceded(tag("x="), parse_num),
            delimited(space0, char(','), space0),
            preceded(tag("y="), parse_num),
        )(s)
    }

    pub fn parse_sensor(s: &str) -> IResult<&str, Sensor> {
        map(
            pair(
                preceded(tag("Sensor at "), parse_point),
                preceded(tag(": closest beacon is at "), parse_point),
            ),
            |(sensor_pt, beacon_pt)| Sensor::new(sensor_pt, beacon_pt),
        )(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_sensor() -> Result<()> {
        assert_eq!(
            Sensor::new((2, 18), (-2, 15)),
            parse_sensor("Sensor at x=2, y=18: closest beacon is at x=-2, y=15")?
        );
        Ok(())
    }
}
