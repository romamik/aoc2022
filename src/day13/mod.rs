use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, Result};
use itertools::{EitherOrBoth, Itertools};
use std::{cmp::Ordering, fmt::Display};

mod input_parser;

type Int = i32;

#[derive(Debug, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Int(Int),
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(list) => {
                write!(f, "[")?;
                for (i, item) in list.iter().enumerate() {
                    write!(f, "{}", item)?;
                    if i != list.len() - 1 {
                        write!(f, ",")?
                    }
                }
                write!(f, "]")?;
            }
            Packet::Int(i) => write!(f, "{}", i)?,
        }
        Ok(())
    }
}

fn cmp_packet_list(a: &[Packet], b: &[Packet]) -> Ordering {
    for pair in a.iter().zip_longest(b.iter()) {
        match pair {
            EitherOrBoth::Both(a, b) => match a.cmp(b) {
                order @ (Ordering::Greater | Ordering::Less) => return order,
                Ordering::Equal => (),
            },
            EitherOrBoth::Left(_) => return Ordering::Greater,
            EitherOrBoth::Right(_) => return Ordering::Less,
        }
    }
    Ordering::Equal
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match [self, other] {
            [Packet::Int(a), Packet::Int(b)] => a.cmp(b),
            [Packet::List(a), Packet::List(b)] => cmp_packet_list(a, b),
            [Packet::List(a), Packet::Int(b)] => cmp_packet_list(a, &[Packet::Int(*b)]),
            [Packet::Int(a), Packet::List(b)] => cmp_packet_list(&[Packet::Int(*a)], b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

type Input = Vec<(Packet, Packet)>;

impl SolutionInput for Input {
    fn parse(input_str: &str) -> Result<Self> {
        input_str
            .split("\n\n")
            .map(|s| {
                s.split('\n')
                    .map(|s| s.parse::<Packet>())
                    .try_collect::<_, Vec<_>, _>()?
                    .into_iter()
                    .collect_tuple()
                    .ok_or_else(|| anyhow!("not a pair {}", s))
            })
            .collect::<Result<Vec<_>, _>>()
    }
}

pub struct Day13Pt1;
impl Solution for Day13Pt1 {
    const DAY: usize = 13;
    const PART: usize = 1;

    type TInput = Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut result = 0;
        for (i, (a, b)) in input.iter().enumerate() {
            if a < b {
                result += i + 1
            }
        }
        Ok(result)
    }
}

pub struct Day13Pt2;
impl Solution for Day13Pt2 {
    const DAY: usize = 13;
    const PART: usize = 2;

    type TInput = Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let dividers = [
            Packet::List(vec![Packet::List(vec![Packet::Int(2)])]),
            Packet::List(vec![Packet::List(vec![Packet::Int(6)])]),
        ];
        let mut all_packets = input
            .iter()
            .flat_map(|pair| [&pair.0, &pair.1].into_iter())
            .chain(dividers.iter())
            .collect_vec();

        all_packets.sort();

        let (i0, i1) = dividers
            .iter()
            .map(|divider| {
                all_packets
                    .iter()
                    .cloned()
                    .find_position(|p| divider == *p)
                    .map(|(i, _)| i)
            })
            .collect_tuple()
            .unwrap();

        let i0 = 1 + i0.ok_or_else(|| anyhow!("divider 1 not found"))?;
        let i1 = 1 + i1.ok_or_else(|| anyhow!("divider 2 not found"))?;

        Ok(i0 * i1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use all_asserts::{assert_gt, assert_lt};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Input = get_input::<Day13Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Input = get_input::<Day13Pt1>("input.txt").unwrap();
    }
    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(21890, Day13Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(140, Day13Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(4821, Day13Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(13, Day13Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_ord() -> Result<()> {
        assert_lt!(
            "[1,1,3,1,1]".parse::<Packet>()?,
            "[1,1,5,1,1]".parse::<Packet>()?,
        );
        assert_lt!(
            "[[1],[2,3,4]]".parse::<Packet>()?,
            "[[1],4]".parse::<Packet>()?,
        );
        assert_gt!("[9]".parse::<Packet>()?, "[[8,7,6]]".parse::<Packet>()?);

        assert_eq!("[4,4]".parse::<Packet>()?, "[4,4]".parse::<Packet>()?);
        assert_lt!("[4,4]".parse::<Packet>()?, "[4,4,4]".parse::<Packet>()?);
        assert_gt!("[4,4,4]".parse::<Packet>()?, "[4,4]".parse::<Packet>()?);
        assert_lt!(
            "[[4,4],4,4]".parse::<Packet>()?,
            "[[4,4],4,4,4]".parse::<Packet>()?
        );

        assert_gt!("[7,7,7,7]".parse::<Packet>()?, "[7,7,7]".parse::<Packet>()?);
        assert_lt!("[]".parse::<Packet>()?, "[3]".parse::<Packet>()?);
        assert_gt!("[[[]]]".parse::<Packet>()?, "[[]]".parse::<Packet>()?);
        assert_gt!(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse::<Packet>()?,
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>()?
        );

        Ok(())
    }
}
