use super::{Coord, Line, Point};
use anyhow::{ensure, Result};

fn parse_line(s: &str) -> Result<Line> {
    let (rem, line) = nom_parser::parse_line(s).map_err(|e| e.to_owned())?;
    ensure!(rem.len() == 0);
    Ok(line)
}

pub fn parse_lines(s: &str) -> Result<Vec<Line>> {
    s.split('\n').map(parse_line).collect()
}

mod nom_parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{char, digit1, space0},
        combinator::map_res,
        multi::separated_list0,
        sequence::{delimited, separated_pair},
        IResult,
    };

    fn parse_num(s: &str) -> IResult<&str, Coord> {
        map_res(digit1, |s: &str| s.parse())(s)
    }

    fn parse_point(s: &str) -> IResult<&str, Point> {
        separated_pair(parse_num, delimited(space0, char(','), space0), parse_num)(s)
    }

    pub fn parse_line(s: &str) -> IResult<&str, Line> {
        separated_list0(delimited(space0, tag("->"), space0), parse_point)(s)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_line;
    use anyhow::Result;

    #[test]
    fn test_parse_line() -> Result<()> {
        assert_eq!(
            &vec![(498, 4), (498, 6), (496, 6)],
            &parse_line("498,4 -> 498,6 -> 496,6")?
        );
        Ok(())
    }
}
