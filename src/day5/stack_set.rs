use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use std::fmt::{Display, Write};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct StackSet {
    stacks: Vec<Vec<char>>,
    names: Vec<char>,
}

fn find_index_by_name(names: &[char], name: char) -> Result<usize> {
    let (index, _) = names
        .iter()
        .cloned()
        .find_position(|&it| it == name)
        .ok_or_else(|| anyhow!("stack with name {:?} not found", name))?;
    Ok(index)
}

impl StackSet {
    pub fn apply_moves(&mut self, count: usize, from: char, to: char) -> Result<()> {
        let from_i = find_index_by_name(&self.names, from)?;
        let to_i = find_index_by_name(&self.names, to)?;
        for i in 0..count {
            let name = self.stacks[from_i]
                .pop()
                .ok_or_else(|| anyhow!("stack {:?} empty, i=={}", from, i))?;
            self.stacks[to_i].push(name);
        }
        Ok(())
    }

    pub fn get_top_names(&self) -> Vec<char> {
        self.stacks
            .iter()
            .filter_map(|it| it.last().cloned())
            .collect_vec()
    }
}

impl Display for StackSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let height = self.stacks.iter().map(|it| it.len()).max().unwrap_or(0);
        for y in (0..height).rev() {
            for (i, stack) in self.stacks.iter().enumerate() {
                match stack.get(y) {
                    Some(name) => f.write_fmt(format_args!("[{}]", name))?,
                    None => f.write_str("   ")?,
                }
                if i != self.stacks.len() - 1 {
                    f.write_char(' ')?;
                }
            }
            f.write_char('\n')?;
        }
        for (i, name) in self.names.iter().enumerate() {
            f.write_fmt(format_args!(" {} ", name))?;
            if i != self.stacks.len() - 1 {
                f.write_char(' ')?;
            }
        }
        Ok(())
    }
}

impl FromStr for StackSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut it = s
            .split('\n')
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        let bottom_line = it.next().ok_or_else(|| anyhow!("no bottom line"))?;

        // bottom_line: " 1 " + " " + " 2 "
        // len = count * 3 + count - 1
        // count = (len + 1) / 4;
        let stack_count = (bottom_line.len() + 1) / 4;
        let names = (0..stack_count)
            .map(|n| bottom_line[n * 4 + 1])
            .collect::<Vec<_>>();

        let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_count];
        for line in it {
            // line: "[A]" + " " + "[B]"
            (0..stack_count).for_each(|n| {
                let &name = line.get(n * 4 + 1).unwrap_or(&' ');
                if name != ' ' {
                    stacks[n].push(name);
                }
            });
        }

        Ok(StackSet { names, stacks })
    }
}

#[cfg(test)]
pub(crate) mod tests {

    use super::StackSet;
    use itertools::Itertools;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref STACK_SET_INPUT: String = [
            "    [D]  ",     //
            "[N] [C]      ", //
            "[Z] [M] [P]", //
            " 1   2   3 ", //
        ]
        .join("\n");
    }

    #[test]
    fn test_parse() {
        fn trim_lines(s: &str) -> String {
            s.split('\n').map(|s| s.trim()).join("\n")
        }

        let stack_set = STACK_SET_INPUT.parse::<StackSet>().unwrap();

        assert_eq!(
            trim_lines(&STACK_SET_INPUT),
            trim_lines(&stack_set.to_string())
        )
    }

    #[test]
    fn test_move() {
        let mut stack_set = STACK_SET_INPUT.parse::<StackSet>().unwrap();

        stack_set.apply_moves(1, '2', '1');
        assert_eq!(
            [
                "[D]        ", //
                "[N] [C]    ", //
                "[Z] [M] [P]", //
                " 1   2   3 ", //
            ]
            .join("\n"),
            stack_set.to_string()
        );

        stack_set.apply_moves(3, '1', '3');
        assert_eq!(
            [
                "        [Z]", //
                "        [N]", //
                "    [C] [D]", //
                "    [M] [P]", //
                " 1   2   3 ", //
            ]
            .join("\n"),
            stack_set.to_string()
        );
    }
}
