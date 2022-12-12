use crate::solution::{Solution, SolutionInput};
use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;
use std::{collections::HashMap, fmt::Debug};

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

trait Item: Debug + Sized {
    fn create(value: Num, divisors: &[Num]) -> Self;
    fn divisible_by(&self, n: Num) -> Result<bool>;
    fn apply_op(&mut self, op: &Op) -> Result<()>;
    fn after_op(&mut self);
}

#[derive(Debug)]
struct Monkey<TItem> {
    _name: MonkeyName,
    items: Vec<TItem>,
    rule: MonkeyRule,
}

impl<TItem: Item> Monkey<TItem> {
    fn create(input: &[MonkeyInput]) -> Vec<Self> {
        let divisors = input
            .iter()
            .map(|monkey| monkey.rule.divisible_by)
            .collect_vec();

        input
            .iter()
            .map(|monkey| Monkey {
                _name: monkey.name,
                items: monkey
                    .starting_items
                    .iter()
                    .map(|&v| TItem::create(v, &divisors))
                    .collect_vec(),
                rule: monkey.rule.clone(),
            })
            .collect_vec()
    }

    fn make_turn(
        monkeys: &mut [Self],
        name: usize,
        inspection_count: &mut HashMap<MonkeyName, usize>,
    ) -> Result<()> {
        let items = std::mem::take(&mut monkeys[name].items);

        *inspection_count.entry(name).or_default() += items.len();

        for mut item in items {
            let monkey = &monkeys[name];
            let rule = &monkey.rule;
            let op = &rule.op;

            item.apply_op(op)?;
            item.after_op();

            let test_result = item.divisible_by(rule.divisible_by)?;

            let dst_monkey = if test_result {
                rule.monkey_if_true
            } else {
                rule.monkey_if_false
            };

            monkeys[dst_monkey].items.push(item);
        }

        Ok(())
    }

    fn make_round(
        monkeys: &mut Vec<Self>,
        inspection_count: &mut HashMap<MonkeyName, usize>,
    ) -> Result<()> {
        for i in 0..monkeys.len() {
            Self::make_turn(monkeys, i, inspection_count)?;
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

type Part1Item = Num;

impl Item for Part1Item {
    fn create(value: Num, _divisors: &[Num]) -> Self {
        value
    }

    fn divisible_by(&self, n: Num) -> Result<bool> {
        Ok(self % n == 0)
    }

    fn apply_op(&mut self, (lhs, op, rhs): &Op) -> Result<()> {
        let lhs = lhs.resolve(*self);
        let rhs = rhs.resolve(*self);
        *self = match op {
            OpType::Add => lhs + rhs,
            OpType::Mul => lhs * rhs,
        };
        Ok(())
    }

    fn after_op(&mut self) {
        *self /= 3
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
        let mut monkeys: Vec<Monkey<Part1Item>> = Monkey::create(input);
        for _ in 0..20 {
            Monkey::make_round(&mut monkeys, &mut inspection_count)?
        }
        let mut counts = inspection_count.iter().collect::<Vec<_>>();
        counts.sort_by(|(_name_a, count_a), (_name_b, count_b)| usize::cmp(count_b, count_a));
        Ok(counts[0].1 * counts[1].1)
    }
}

type Part2Item = HashMap<Num, Num>; // Divisor => Value

fn get_value_for_divisor(item: &Part2Item, divisor: Num) -> Result<Num> {
    item.get(&divisor)
        .cloned()
        .ok_or_else(|| anyhow!("no value for divisor {}", divisor))
}

impl Item for Part2Item {
    fn create(value: Num, divisors: &[Num]) -> Self {
        divisors
            .iter()
            .map(|&divisor| (divisor, value % divisor))
            .collect()
    }

    fn divisible_by(&self, divisor: Num) -> Result<bool> {
        Ok(get_value_for_divisor(self, divisor)? % divisor == 0)
    }

    fn apply_op(&mut self, op: &Op) -> Result<()> {
        for (divisor, value) in self.iter_mut() {
            Num::apply_op(value, op)?;
            *value %= divisor;
        }
        Ok(())
    }

    fn after_op(&mut self) {}
}

pub struct Day11Pt2;

impl Solution for Day11Pt2 {
    const DAY: usize = 11;
    const PART: usize = 2;

    type TInput = Vec<MonkeyInput>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut inspection_count: HashMap<MonkeyName, usize> = HashMap::new();
        let mut monkeys: Vec<Monkey<Part2Item>> = Monkey::create(input);
        for _ in 0..10000 {
            Monkey::make_round(&mut monkeys, &mut inspection_count)?
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
        let mut monkeys: Vec<Monkey<Part1Item>> = Monkey::create(&INPUT_TEST);
        Monkey::make_round(&mut monkeys, &mut HashMap::new());
        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
        Ok(())
    }
}
