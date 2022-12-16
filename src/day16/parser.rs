use anyhow::{anyhow, ensure, Context, Result};

use super::*;

pub fn parse_room(s: &str) -> Result<(String, Room)> {
    let (rem, (name, room)) = nom_parser::parse_room(s).map_err(|e| e.to_owned())?;
    ensure!(rem.len() == 0);
    Ok((name, room))
}

pub fn parse_system(s: &str) -> Result<System> {
    let rooms = s
        .split('\n')
        .map(|s| parse_room(s).with_context(|| anyhow!("parsing {:?}", s)))
        .collect::<Result<Vec<_>>>()?;
    let starting_room = rooms
        .first()
        .ok_or_else(|| anyhow!("no rooms!"))?
        .0
        .to_owned();
    let rooms = rooms.into_iter().collect::<HashMap<_, _>>();
    Ok(System {
        starting_room,
        rooms,
    })
}

mod nom_parser {

    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, char, digit1, space0},
        combinator::{map, map_res, recognize},
        multi::separated_list0,
        sequence::{preceded, tuple},
        IResult,
    };

    fn parse_num(s: &str) -> IResult<&str, usize> {
        map_res(recognize(digit1), |s: &str| s.parse())(s)
    }

    fn parse_name(s: &str) -> IResult<&str, String> {
        map(alpha1, |s: &str| s.to_string())(s)
    }

    fn parse_name_list(s: &str) -> IResult<&str, Vec<String>> {
        separated_list0(tuple((space0, char(','), space0)), parse_name)(s)
    }

    pub fn parse_room(s: &str) -> IResult<&str, (String, Room)> {
        map(
            tuple((
                preceded(tag("Valve "), parse_name),
                preceded(tag(" has flow rate="), parse_num),
                preceded(tag("; tunnels lead to valves "), parse_name_list),
            )),
            |(name, flow, tunnels)| (name, Room { flow, tunnels }),
        )(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_system() -> Result<()> {
        assert_eq!(
            System {
                starting_room: "AA".to_string(),
                rooms: [
                    (
                        "AA".to_string(),
                        Room {
                            flow: 0,
                            tunnels: vec!["DD".to_string(), "II".to_string(), "BB".to_string()]
                        }
                    ),
                    (
                        "BB".to_string(),
                        Room {
                            flow: 13,
                            tunnels: vec!["CC".to_string(), "AA".to_string()]
                        }
                    )
                ]
                .into_iter()
                .collect()
            },
            parse_system("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA")?
        );
        Ok(())
    }
}
