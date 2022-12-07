use anyhow::{anyhow, bail, Context, Result};

use crate::solution::{Solution, SolutionInput};

use self::fs::{Dir, FSEntry, File};

pub mod fs;

type Day7Input = Vec<Command>;

pub struct Day7Pt1;

fn visit_all<FFile: FnMut(&File), FDir: FnMut(&Dir)>(
    entry: &FSEntry,
    fn_file: &mut FFile,
    fn_dir: &mut FDir,
) {
    if let Ok(file) = entry.file() {
        fn_file(file)
    } else if let Ok(dir) = entry.dir() {
        fn_dir(dir);
        for (_, entry) in dir.entries() {
            visit_all(entry, fn_file, fn_dir);
        }
    }
}

impl Solution for Day7Pt1 {
    const DAY: usize = 7;
    const PART: usize = 1;

    type TInput = Day7Input;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut root = Dir::new();
        run_commands(&mut root, &mut input.iter())?;

        let mut total_size = 0;
        visit_all(&FSEntry::Dir(root), &mut |_| (), &mut |dir| {
            let size = dir.size();
            if size < 100_000 {
                total_size += size;
            }
        });

        Ok(total_size)
    }
}

#[derive(Debug)]
pub enum Command {
    MkDir { name: String },
    AddFile { name: String, size: usize },
    ChangeDir { name: String },
}

impl SolutionInput for Day7Input {
    fn parse(input_str: &str) -> Result<Self> {
        let mut result: Vec<Command> = Vec::new();

        for cmd in input_str.split("$ ").map(str::trim) {
            let lines: Vec<Vec<&str>> = cmd
                .split('\n')
                .map(|line| line.split(' ').collect::<Vec<_>>())
                .collect::<Vec<_>>();

            if let Some((cmd, lines)) = lines.split_first() {
                match cmd[..] {
                    ["cd", "/"] | [""] | [] => (),

                    ["cd", name] => result.push(Command::ChangeDir {
                        name: name.to_string(),
                    }),

                    ["ls"] => {
                        for line in lines {
                            let command: Command = match line[..] {
                                ["dir", name] => Command::MkDir {
                                    name: name.to_string(),
                                },

                                [size_str, name] => {
                                    let size: usize = size_str.parse().with_context(|| {
                                        anyhow!(
                                            "parsing size, name: {:?}, size: {:?}",
                                            name,
                                            size_str
                                        )
                                    })?;
                                    Command::AddFile {
                                        name: name.to_string(),
                                        size,
                                    }
                                }

                                _ => bail!("unexpected ls item {:?}", line),
                            };

                            result.push(command);
                        }
                    }

                    _ => bail!("unexpected command {:?}", cmd),
                }
            }
        }

        Ok(result)
    }
}

fn run_commands<'a, T: Iterator<Item = &'a Command>>(
    current_dir: &mut Dir,
    command_iter: &mut T,
) -> Result<()> {
    while let Some(command) = command_iter.next() {
        match command {
            Command::MkDir { name } => {
                current_dir.add_dir(name.as_str())?;
            }
            Command::AddFile { name, size } => {
                current_dir.add_file(name.as_str(), *size)?;
            }
            Command::ChangeDir { ref name } if name == ".." => return Ok(()),
            Command::ChangeDir { name } => {
                let dir = current_dir
                    .dir_mut(name)
                    .ok_or_else(|| anyhow!("can`t ChangeDir, no dir with name {:?}", name))?;
                run_commands(dir, command_iter).with_context(|| anyhow!("in dir {:?}", name))?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Day7Input = get_input::<Day7Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Day7Input = get_input::<Day7Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(1428881, Day7Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(95437, Day7Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_parse() -> Result<()> {
        let mut root = Dir::new();
        run_commands(&mut root, &mut INPUT_TEST.iter())?;

        assert_eq!(584, root.dir_mut("a").unwrap().dir_mut("e").unwrap().size());
        assert_eq!(
            4060174 + 8033020 + 5626152 + 7214296,
            root.dir_mut("d").unwrap().size()
        );
        Ok(())
    }

    #[test]
    fn test_simple() -> Result<()> {
        let mut root = Dir::new();
        run_commands(
            &mut root,
            &mut [
                Command::MkDir {
                    name: "foo".to_string(),
                },
                Command::AddFile {
                    name: "some_file_in_root".to_string(),
                    size: 100,
                },
                Command::ChangeDir {
                    name: "foo".to_string(),
                },
                Command::AddFile {
                    name: "file_in_foo".to_string(),
                    size: 100,
                },
                Command::ChangeDir {
                    name: "..".to_string(),
                },
                Command::AddFile {
                    name: "some_other_file_in_root".to_string(),
                    size: 100,
                },
            ]
            .iter(),
        )?;

        assert_eq!(300, root.size());
        assert_eq!(100, root.dir_mut("foo").unwrap().size());

        println!("{:?}", root);
        Ok(())
    }
}
