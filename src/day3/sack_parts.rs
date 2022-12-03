use anyhow::bail;

use anyhow::Error;
use anyhow::Result;
use itertools::Itertools;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::str::FromStr;

use super::item::Item;
use super::item::MAX_PRIORITY;

pub struct SackParts(Vec<Item>, Vec<Item>);

impl SackParts {
    pub fn find_duplicate_item(&self) -> Result<Item> {
        let mut usages: [bool; MAX_PRIORITY + 1] = [false; MAX_PRIORITY + 1];
        self.0
            .iter()
            .for_each(|item| usages[item.priority()] = true);
        for item in self.1.iter() {
            if usages[item.priority()] {
                return Ok(*item);
            }
        }
        bail!("duplicate item not found {:?}", self)
    }
}

impl Debug for SackParts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let [s0, s1] = [&self.0, &self.1].map(|v| v.iter().map(|item| item.as_char()).join(""));
        f.write_fmt(format_args!("SackContent({}, {})", &s0, &s1))?;
        Ok(())
    }
}

impl FromStr for SackParts {
    type Err = Error;
    fn from_str(input_str: &str) -> Result<Self> {
        if input_str.len() % 2 != 0 {
            bail!("odd length {}", input_str.len());
        }
        let len = input_str.len() / 2;
        let [part1, part2] = [&input_str[..len], &input_str[len..]].map(|rng| {
            rng.as_bytes()
                .iter()
                .map(|&code| Item::from_char_code(code))
                .collect::<Result<Vec<Item>>>()
        });
        let sack = SackParts(part1?, part2?);
        assert_eq!(len, sack.0.len());
        assert_eq!(len, sack.1.len());
        Ok(sack)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let sack_content = "npbB".parse::<SackParts>().unwrap();
        assert_eq!(
            &vec![
                Item::from_char_code(b'n').unwrap(),
                Item::from_char_code(b'p').unwrap()
            ],
            &sack_content.0
        );
        assert_eq!(
            &vec![
                Item::from_char_code(b'b').unwrap(),
                Item::from_char_code(b'B').unwrap()
            ],
            &sack_content.1
        );
    }

    #[test]
    fn test_find_dup() {
        let sack_content = "helloHELLo".parse::<SackParts>().unwrap();
        assert_eq!('o', sack_content.find_duplicate_item().unwrap().as_char());
    }
}
