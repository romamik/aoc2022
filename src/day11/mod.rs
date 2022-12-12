use std::collections::HashMap;

use anyhow::{anyhow, ensure, Result};

use crate::solution::{Solution, SolutionInput};

mod input_parser;
type Num = i32;

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

impl OpType {
    fn apply(&self, lhs: &OpVal, rhs: &OpVal, old: Num) -> Num {
        let lhs = lhs.resolve(old);
        let rhs = rhs.resolve(old);
        match self {
            OpType::Add => lhs + rhs,
            OpType::Mul => lhs * rhs,
        }
    }
}

type Op = (OpVal, OpType, OpVal);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Test {
    DivisibleBy(Num),
}

type MonkeyName = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MonkeyRule {
    op: Op,
    test: Test,
    monkey_if_true: MonkeyName,
    monkey_if_false: MonkeyName,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    name: MonkeyName,
    items: Vec<Num>,
    rule: MonkeyRule,
}

fn make_turn(
    monkeys: &mut Vec<Monkey>,
    name: usize,
    inspection_count: &mut HashMap<MonkeyName, usize>,
) {
    let items = std::mem::take(&mut monkeys[name].items);

    *inspection_count.entry(name).or_default() += items.len();

    for item in items {
        let monkey = &monkeys[name];
        let rule = &monkey.rule;
        let op = &rule.op;

        let new_item = op.1.apply(&op.0, &op.2, item) / 3;

        let test_result = match rule.test {
            Test::DivisibleBy(d) => new_item % d == 0,
        };

        let dst_monkey = if test_result {
            rule.monkey_if_true
        } else {
            rule.monkey_if_false
        };

        monkeys[dst_monkey].items.push(new_item);
    }
}

fn make_round(monkeys: &mut Vec<Monkey>, inspection_count: &mut HashMap<MonkeyName, usize>) {
    for i in 0..monkeys.len() {
        make_turn(monkeys, i, inspection_count);
    }
}

impl SolutionInput for Vec<Monkey> {
    fn parse(input_str: &str) -> Result<Self> {
        let (rem, list) = Monkey::parse_list(input_str).map_err(|e| anyhow!("{:?}", e))?;
        ensure!(rem.len() == 0);
        Ok(list)
    }
}

pub struct Day11Pt1;

impl Solution for Day11Pt1 {
    const DAY: usize = 11;
    const PART: usize = 1;

    type TInput = Vec<Monkey>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut inspection_count: HashMap<MonkeyName, usize> = HashMap::new();
        let mut monkeys = input.clone();
        for _ in 0..20 {
            make_round(&mut monkeys, &mut inspection_count)
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
        static ref INPUT_TEST: Vec<Monkey> = get_input::<Day11Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Monkey> = get_input::<Day11Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(10605, Day11Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_make_round() -> Result<()> {
        let mut monkeys = INPUT_TEST.clone();
        make_round(&mut monkeys, &mut HashMap::new());
        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
        Ok(())
    }
}
