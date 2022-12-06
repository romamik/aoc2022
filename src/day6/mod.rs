use std::collections::{HashMap, VecDeque};

use anyhow::bail;

use crate::solution::{Solution, SolutionInput};

type Day6Input = String;

impl SolutionInput for Day6Input {
    fn parse(input_str: &str) -> anyhow::Result<Self> {
        Ok(input_str.to_owned())
    }
}

pub struct Day6Pt1;

impl Solution for Day6Pt1 {
    const DAY: usize = 6;
    const PART: usize = 1;

    type TInput = Day6Input;
    type TOutput = usize;

    fn solve(input: &String) -> anyhow::Result<usize> {
        let mut char_count: HashMap<char, usize> = HashMap::new();
        let mut char_queue: VecDeque<char> = VecDeque::new();
        let mut duplicates: usize = 0;
        for (i, c) in input.chars().enumerate() {
            // insert new char
            char_queue.push_back(c);
            let count: &mut usize = char_count.entry(c).or_default();
            *count += 1;
            if *count == 2 {
                duplicates += 1;
            }

            // remove char
            if char_queue.len() > 4 {
                let c = char_queue.pop_front().unwrap();
                let count: &mut usize = char_count.get_mut(&c).unwrap();
                *count -= 1;
                if *count == 1 {
                    duplicates -= 1;
                }
            }

            // check condition
            if char_queue.len() == 4 && duplicates == 0 {
                return Ok(i + 1);
            }
        }

        bail!("not found");
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_MAIN: Day6Input = get_input::<Day6Pt1>("input.txt").unwrap();
    }

    // #[test]
    // fn test_pt2_result() {
    //     assert_eq!("VHJDDCWRD", &Day5Pt2::solve(&INPUT_MAIN).unwrap());
    // }

    // #[test]
    // fn test_pt2() {
    //     assert_eq!("MCD", &Day5Pt2::solve(&INPUT_TEST).unwrap());
    // }

    #[test]
    fn test_pt1_result() {
        assert_eq!(1804, Day6Pt1::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt1() {
        fn run(s: &str) -> usize {
            Day6Pt1::solve(&s.to_owned()).unwrap()
        }

        assert_eq!(7, run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(5, run("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, run("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}
