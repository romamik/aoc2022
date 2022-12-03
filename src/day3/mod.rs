mod item;
mod pt1;
mod sack_parts;

use self::item::{Item, MAX_PRIORITY};
use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, Result};
use itertools::Itertools;
pub use pt1::Day3Pt1;
use std::collections::HashSet;

pub struct Day3Pt2;

type Group = [Vec<Item>; 3];
type Day3Pt2Input = Vec<Group>;

fn find_common_item(group: &Group) -> Result<Item> {
    let mut counts: [usize; MAX_PRIORITY + 1] = [0; MAX_PRIORITY + 1];
    for sack in group.iter() {
        let hash = sack.iter().collect::<HashSet<_>>();
        for item in hash.iter() {
            counts[item.priority()] += 1;
        }
    }
    let (priority, _) = counts
        .iter()
        .cloned()
        .find_position(|&c| c == group.len())
        .ok_or_else(|| anyhow!("common item not found"))?;
    Item::from_priority(priority)
}

impl SolutionInput for Day3Pt2Input {
    fn parse(input_str: &str) -> anyhow::Result<Self> {
        let tuples_iter = input_str
            .split('\n')
            .map(|s| {
                s.as_bytes()
                    .iter()
                    .map(|&it| Item::from_char_code(it))
                    .collect::<Result<Vec<_>>>()
            })
            .tuples::<(_, _, _)>();

        //TODO check noting behind if line count not divisible by 3
        tuples_iter
            .map(|tup| Ok([tup.0?, tup.1?, tup.2?]))
            .collect::<Result<Vec<_>>>()
    }
}

impl Solution for Day3Pt2 {
    const DAY: usize = 3;
    const PART: usize = 2;

    type TInput = Day3Pt2Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        Ok(input
            .iter()
            .map(|group| Ok(find_common_item(group)?.priority()))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Day3Pt2Input = get_input::<Day3Pt2>("test.txt").unwrap();
        static ref INPUT_MAIN: Day3Pt2Input = get_input::<Day3Pt2>("input.txt").unwrap();
    }

    #[test]
    fn test_pt2_result() {
        assert_eq!(2525, Day3Pt2::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt2() {
        assert_eq!(70, Day3Pt2::solve(&INPUT_TEST).unwrap());
    }

    #[test]
    fn test_find_common() {
        assert_eq!(
            'a',
            find_common_item(&Day3Pt2Input::parse("ab\nac\nad").unwrap()[0])
                .unwrap()
                .as_char()
        );

        assert_eq!(
            'b',
            find_common_item(&Day3Pt2Input::parse("aaab\ncb\ndb").unwrap()[0])
                .unwrap()
                .as_char()
        );

        assert_eq!('r', find_common_item(&INPUT_TEST[0]).unwrap().as_char())
    }
}
