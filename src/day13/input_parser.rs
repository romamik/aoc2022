use super::Packet;
use anyhow::{ensure, Error};
use nom::{
    branch::alt,
    character::complete::{char, digit1, space0},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::str::FromStr;

impl Packet {
    fn parse(s: &str) -> IResult<&str, Packet> {
        alt((Self::parse_int, Self::parse_list))(s)
    }

    fn parse_int(s: &str) -> IResult<&str, Packet> {
        map(map_res(digit1, |s: &str| s.parse()), Packet::Int)(s)
    }

    fn parse_list(s: &str) -> IResult<&str, Packet> {
        map(
            delimited(
                char('['),
                separated_list0(delimited(space0, char(','), space0), Packet::parse),
                char(']'),
            ),
            Packet::List,
        )(s)
    }
}

impl FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rem, packet) = Packet::parse(s).map_err(|e| e.to_owned())?;
        ensure!(rem.len() == 0);
        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::day13::Packet;

    #[test]
    fn test() -> Result<()> {
        use Packet::*;

        assert_eq!(Int(100), Packet::parse("100")?.1);
        assert_eq!(
            List(vec![Int(100), Int(200)]),
            Packet::parse("[100, 200]")?.1
        );
        assert_eq!(
            List(vec![List(vec![Int(100), Int(200)]), Int(300)]),
            Packet::parse("[[100, 200], 300]")?.1
        );

        Ok(())
    }
}
