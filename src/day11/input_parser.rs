use super::*;
use anyhow::{bail, Error, Result};
use nom::{
    bytes::complete::{tag, take},
    character::{
        complete::{digit1, multispace0, multispace1, one_of},
        streaming::{space0, space1},
    },
    combinator::{map, map_res, opt, recognize},
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};
use std::str::FromStr;

impl OpVal {
    fn parse_old(s: &str) -> IResult<&str, OpVal> {
        map(tag("old"), |_| OpVal::Old)(s)
    }

    fn parse_num(s: &str) -> IResult<&str, OpVal> {
        map_res(
            recognize(tuple((opt(one_of("+-")), digit1))),
            |s: &str| -> Result<OpVal> { Ok(OpVal::Num(s.parse()?)) },
        )(s)
    }

    pub fn parse(s: &str) -> IResult<&str, OpVal> {
        nom::branch::alt((Self::parse_old, Self::parse_num))(s)
    }
}

impl FromStr for OpType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => OpType::Add,
            "*" => OpType::Mul,
            _ => bail!("unexpected OpType {:?}", s),
        })
    }
}

impl OpType {
    pub fn parse(s: &str) -> IResult<&str, OpType> {
        map_res(take(1_usize), |s: &str| s.parse())(s)
    }
}

pub fn parse_op(s: &str) -> IResult<&str, Op> {
    tuple((
        OpVal::parse,
        preceded(space0, OpType::parse),
        preceded(space0, OpVal::parse),
    ))(s)
}

impl Test {
    fn parse(s: &str) -> IResult<&str, Test> {
        preceded(
            pair(tag("divisible by"), space1),
            map_res(digit1, |s: &str| -> Result<_, Error> {
                Ok(Test::DivisibleBy(s.parse::<Num>()?))
            }),
        )(s)
    }
}

pub fn spaced_tag<'a, Error: ParseError<&'a str>>(
    t: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str, Error> {
    delimited(multispace0, tag(t), multispace0)
}

impl Monkey {
    fn parse_monkey_name(s: &str) -> IResult<&str, MonkeyName> {
        map_res(digit1, |s: &str| s.parse())(s)
    }

    fn parse_item(s: &str) -> IResult<&str, Num> {
        map_res(digit1, |s: &str| s.parse())(s)
    }

    fn parse_items(s: &str) -> IResult<&str, Vec<Num>> {
        separated_list0(tuple((space0, tag(","), space0)), Self::parse_item)(s)
    }

    pub fn parse(s: &str) -> IResult<&str, Monkey> {
        map(
            tuple((
                delimited(
                    spaced_tag("Monkey"),
                    Self::parse_monkey_name,
                    spaced_tag(":"),
                ),
                preceded(spaced_tag("Starting items:"), Self::parse_items),
                preceded(spaced_tag("Operation: new ="), parse_op),
                preceded(spaced_tag("Test:"), Test::parse),
                preceded(
                    spaced_tag("If true: throw to monkey"),
                    Self::parse_monkey_name,
                ),
                preceded(
                    spaced_tag("If false: throw to monkey"),
                    Self::parse_monkey_name,
                ),
            )),
            |(name, items, op, test, monkey_if_true, monkey_if_false)| Monkey {
                name,
                items,
                rule: MonkeyRule {
                    op,
                    test,
                    monkey_if_true,
                    monkey_if_false,
                },
            },
        )(s)
    }

    pub fn parse_list(s: &str) -> IResult<&str, Vec<Monkey>> {
        map_res(separated_list0(multispace1, Self::parse), |monkeys| {
            for (i, monkey) in monkeys.iter().enumerate() {
                ensure!(i == monkey.name);
                ensure!(monkey.rule.monkey_if_true < monkeys.len());
                ensure!(monkey.rule.monkey_if_false < monkeys.len());
            }
            Ok(monkeys)
        })(s)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_monkey_list() {
        let s = [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
        ]
        .join("\n");
        assert_eq!(
            Monkey::parse_list(&s),
            Ok((
                "",
                vec![
                    Monkey {
                        name: 0,
                        items: vec![79, 98],
                        rule: MonkeyRule {
                            op: (OpVal::Old, OpType::Mul, OpVal::Num(19)),
                            test: Test::DivisibleBy(23),
                            monkey_if_true: 2,
                            monkey_if_false: 3
                        }
                    },
                    Monkey {
                        name: 1,
                        items: vec![79, 98],
                        rule: MonkeyRule {
                            op: (OpVal::Old, OpType::Mul, OpVal::Num(19)),
                            test: Test::DivisibleBy(23),
                            monkey_if_true: 2,
                            monkey_if_false: 3
                        }
                    },
                ]
            ))
        )
    }

    #[test]
    fn test_parse_monkey() {
        let s = [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
        ]
        .join("\n");
        assert_eq!(
            Monkey::parse(&s),
            Ok((
                "",
                Monkey {
                    name: 0,
                    items: vec![79, 98],
                    rule: MonkeyRule {
                        op: (OpVal::Old, OpType::Mul, OpVal::Num(19)),
                        test: Test::DivisibleBy(23),
                        monkey_if_true: 2,
                        monkey_if_false: 3
                    }
                }
            ))
        )
    }

    #[test]
    fn test_parse_op() {
        use OpType::*;
        use OpVal::*;

        assert_eq!(Ok(("", (Num(1), Add, Num(2)))), parse_op("1 + 2"));
        assert_eq!(Ok(("", (Old, Mul, Old))), parse_op("old    *\told"));
        assert_eq!(Ok(("", (Old, Add, Num(3)))), parse_op("old+3"));
        assert!(matches!(parse_op("foo + bar"), Err(_)));
    }

    #[test]
    fn parse_op_type() {
        assert_eq!(Ok(("", OpType::Add)), OpType::parse("+"));
        assert_eq!(Ok(("", OpType::Mul)), OpType::parse("*"));
        assert!(matches!(OpType::parse("foo"), Err(_)));
    }

    #[test]
    fn parse_op_val() {
        assert_eq!(Ok(("", OpVal::Old)), OpVal::parse_old("old"));
        assert!(matches!(OpVal::parse_old("foo"), Err(_)));

        assert_eq!(Ok(("", OpVal::Num(123))), OpVal::parse_num("123"));
        assert_eq!(Ok(("", OpVal::Num(-456))), OpVal::parse_num("-456"));
        assert_eq!(Ok(("", OpVal::Num(789))), OpVal::parse_num("+789"));
        assert!(matches!(
            OpVal::parse_num("123456789121232121433253553"),
            Err(_)
        )); // overflow
        assert!(matches!(OpVal::parse_num("foo"), Err(_)));

        assert_eq!(Ok(("", OpVal::Old)), OpVal::parse("old"));
        assert_eq!(Ok(("", OpVal::Num(123))), OpVal::parse("123"));
    }
}
