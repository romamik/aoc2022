use crate::solution::{Solution, SolutionInput};
use anyhow::{bail, Result};
use std::collections::{HashMap, VecDeque};

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
        find_first_window_with_all_distinct_chars(input, 4)
    }
}

pub struct Day6Pt2;

impl Solution for Day6Pt2 {
    const DAY: usize = 6;
    const PART: usize = 2;

    type TInput = Day6Input;
    type TOutput = usize;

    fn solve(input: &String) -> anyhow::Result<usize> {
        find_first_window_with_all_distinct_chars(input, 14)
    }
}

fn find_first_window_with_all_distinct_chars(input: &str, window_size: usize) -> Result<usize> {
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
        if char_queue.len() > window_size {
            let c = char_queue.pop_front().unwrap();
            let count: &mut usize = char_count.get_mut(&c).unwrap();
            *count -= 1;
            if *count == 1 {
                duplicates -= 1;
            }
        }

        // check condition
        if char_queue.len() == window_size && duplicates == 0 {
            return Ok(i + 1);
        }
    }

    bail!(
        "window with distinct chars not found. window_size {}, input {:?}",
        window_size,
        input
    );
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_MAIN: Day6Input = get_input::<Day6Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_pt2_result() {
        assert_eq!(2508, Day6Pt2::solve(&INPUT_MAIN).unwrap());
    }

    #[test]
    fn test_pt2() {
        fn run(s: &str) -> usize {
            Day6Pt2::solve(&s.to_owned()).unwrap()
        }

        assert_eq!(19, run("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, run("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, run("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

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
