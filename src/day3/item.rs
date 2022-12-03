use anyhow::bail;
use anyhow::Result;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Write;

const RANGES: [[[u8; 2]; 2]; 2] = [[[b'a', b'z'], [1, 26]], [[b'A', b'Z'], [27, 52]]];
pub const _MIN_PRIORITY: usize = RANGES[0][1][0] as usize;
pub const MAX_PRIORITY: usize = RANGES[1][1][1] as usize;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Item {
    priority: u8,
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl Item {
    pub fn from_char_code(code: u8) -> Result<Item> {
        for rng in RANGES.iter() {
            let code_rng = rng[0];
            let priority_rng = rng[1];
            if (code_rng[0]..=code_rng[1]).contains(&code) {
                return Ok(Item {
                    priority: code + priority_rng[0] - code_rng[0],
                });
            }
        }
        bail!("char not in range {:?} ({:?})", code as char, code);
    }
    pub fn priority(&self) -> usize {
        self.priority as usize
    }
    pub fn as_char(&self) -> char {
        for rng in RANGES.iter() {
            let code_rng = rng[0];
            let priority_rng = rng[1];
            if (priority_rng[0]..=priority_rng[1]).contains(&self.priority) {
                return (self.priority + code_rng[0] - priority_rng[0]) as char;
            }
        }
        panic!("priority not in range {:?}", self.priority);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_item() {
        for d in [0, b'a' - b'A'] {
            for c in b'A'..=b'Z' {
                let item = Item::from_char_code(c + d).unwrap();
                assert!((_MIN_PRIORITY..=MAX_PRIORITY).contains(&item.priority()));
                assert_eq!((c + d) as char, item.as_char());
            }
        }
    }
}
