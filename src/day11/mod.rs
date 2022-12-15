use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

mod input_parser;

type Num = i64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OpVal {
    Old,
    Num(Num),
}

impl OpVal {
    fn resolve(&self, old: Num) -> Num {
        match self {
            Self::Old => old,
            Self::Num(num) => *num,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OpType {
    Add,
    Mul,
}

type Op = (OpVal, OpType, OpVal);

type MonkeyName = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MonkeyRule {
    op: Op,
    divisible_by: Num,
    monkey_if_true: MonkeyName,
    monkey_if_false: MonkeyName,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MonkeyInput {
    name: MonkeyName,
    starting_items: Vec<Num>,
    rule: MonkeyRule,
}

fn apply_op_to_item(item: &mut Num, (lhs, op, rhs): &Op) -> Result<()> {
    let lhs = lhs.resolve(*item);
    let rhs = rhs.resolve(*item);
    *item = match op {
        OpType::Add => lhs + rhs,
        OpType::Mul => lhs * rhs,
    };
    Ok(())
}

#[derive(Debug)]
struct Monkey {
    _name: MonkeyName,
    items: Vec<Num>,
    rule: MonkeyRule,
}

#[derive(Debug)]
struct MonkeySet {
    monkeys: Vec<Monkey>,
}

impl MonkeySet {
    fn create(input: &[MonkeyInput]) -> MonkeySet {
        let monkeys = input
            .iter()
            .map(|monkey| Monkey {
                _name: monkey.name,
                items: monkey.starting_items.clone(),
                rule: monkey.rule.clone(),
            })
            .collect_vec();

        MonkeySet { monkeys }
    }

    fn make_turn<F: FnMut(&mut Num)>(
        &mut self,
        name: usize,
        inspection_count: &mut HashMap<MonkeyName, usize>,
        after_op: &mut F,
    ) -> Result<()> {
        let monkeys = &mut self.monkeys;

        let items = std::mem::take(&mut monkeys[name].items);

        *inspection_count.entry(name).or_default() += items.len();

        for mut item in items {
            let monkey = &monkeys[name];
            let rule = &monkey.rule;
            let op = &rule.op;

            apply_op_to_item(&mut item, op)?;
            after_op(&mut item);

            let test_result = item % rule.divisible_by == 0;

            let dst_monkey = if test_result {
                rule.monkey_if_true
            } else {
                rule.monkey_if_false
            };

            monkeys[dst_monkey].items.push(item);
        }

        Ok(())
    }

    fn make_round<F: FnMut(&mut Num)>(
        &mut self,
        inspection_count: &mut HashMap<MonkeyName, usize>,
        after_op: &mut F,
    ) -> Result<()> {
        for i in 0..self.monkeys.len() {
            self.make_turn(i, inspection_count, after_op)?;
        }
        Ok(())
    }
}

impl SolutionInput for Vec<MonkeyInput> {
    fn parse(input_str: &str) -> Result<Self> {
        let (rem, list) = MonkeyInput::parse_list(input_str).map_err(|e| anyhow!("{:?}", e))?;
        ensure!(rem.len() == 0);
        Ok(list)
    }
}

pub struct Day11Pt1;

impl Solution for Day11Pt1 {
    const DAY: usize = 11;
    const PART: usize = 1;

    type TInput = Vec<MonkeyInput>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut inspection_count: HashMap<MonkeyName, usize> = HashMap::new();
        let mut monkeys: MonkeySet = MonkeySet::create(input);
        for _ in 0..20 {
            monkeys.make_round(&mut inspection_count, &mut |item: &mut Num| *item /= 3)?
        }
        let mut counts = inspection_count.iter().collect::<Vec<_>>();
        counts.sort_by(|(_name_a, count_a), (_name_b, count_b)| usize::cmp(count_b, count_a));
        Ok(counts[0].1 * counts[1].1)
    }
}

fn find_divisor(input: &[MonkeyInput]) -> Num {
    let distinct_divisors = input
        .iter()
        .map(|monkey| monkey.rule.divisible_by)
        .collect::<HashSet<_>>()
        .iter()
        .cloned()
        .collect_vec();

    distinct_divisors.iter().product()
}

pub struct Day11Pt2;

impl Solution for Day11Pt2 {
    const DAY: usize = 11;
    const PART: usize = 2;

    type TInput = Vec<MonkeyInput>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut inspection_count: HashMap<MonkeyName, usize> = HashMap::new();
        let mut monkeys: MonkeySet = MonkeySet::create(input);
        let divisor = find_divisor(input);
        for _ in 0..10000 {
            monkeys.make_round(&mut inspection_count, &mut |item| *item %= divisor)?
        }
        let mut counts = inspection_count.iter().collect::<Vec<_>>();
        counts.sort_by(|(_name_a, count_a), (_name_b, count_b)| usize::cmp(count_b, count_a));
        Ok(counts[0].1 * counts[1].1)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<MonkeyInput> = get_input::<Day11Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<MonkeyInput> = get_input::<Day11Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(23612457316, Day11Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(2713310158, Day11Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(110264, Day11Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(10605, Day11Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_make_round() -> Result<()> {
        let mut m: MonkeySet = MonkeySet::create(&INPUT_TEST);
        m.make_round(&mut HashMap::new(), &mut |item: &mut Num| *item /= 3)?;
        assert_eq!(m.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(m.monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(m.monkeys[2].items, vec![]);
        assert_eq!(m.monkeys[3].items, vec![]);
        Ok(())
    }
}
