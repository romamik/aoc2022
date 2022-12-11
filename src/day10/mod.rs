use crate::{
    solution::{Solution, SolutionInput},
    util::split_parse,
};
use anyhow::{anyhow, bail, Context, Error, Result};
use std::io::Write;
use std::str::FromStr;

pub struct State {
    pub x: i32,
}

impl State {
    fn new() -> State {
        State { x: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cmd {
    NoOp,
    AddX(i32),
}

impl Cmd {
    pub fn duration(&self) -> usize {
        match self {
            Cmd::NoOp => 1,
            Cmd::AddX(_) => 2,
        }
    }
    pub fn apply(&self, state: &mut State) {
        match self {
            Cmd::NoOp => {}
            Cmd::AddX(x) => state.x += x,
        }
    }
}

impl FromStr for Cmd {
    type Err = Error;

    fn from_str(s: &str) -> Result<Cmd> {
        let words = s.split(' ').collect::<Vec<_>>();
        let cmd = match words[..] {
            ["noop"] => Cmd::NoOp,
            ["addx", x_str] => Cmd::AddX(
                x_str
                    .parse()
                    .with_context(|| anyhow!("parsing {:?}", x_str))?,
            ),
            _ => bail!("can't parse {:?}", s),
        };
        Ok(cmd)
    }
}

struct VM<TProgram> {
    program: TProgram,
    state: State,
    current_cmd: Option<Cmd>,
    current_cmd_end_cycle: usize,
    signal_strength: i32,
    completed_cycles: usize,
}

impl<TProgram: Iterator<Item = Cmd>> VM<TProgram> {
    pub fn new(program: TProgram) -> VM<TProgram> {
        let mut vm = VM {
            program,
            state: State::new(),
            current_cmd: None,
            current_cmd_end_cycle: 0,
            signal_strength: 0,
            completed_cycles: 0,
        };
        vm.take_next_cmd();
        vm
    }

    pub fn cycle(&mut self) {
        self.completed_cycles += 1;
        self.signal_strength = self.completed_cycles as i32 * self.state.x;

        if let Some(cmd) = self.current_cmd {
            if self.completed_cycles == self.current_cmd_end_cycle {
                cmd.apply(&mut self.state);
                self.take_next_cmd();
            }
        }
    }

    pub fn cycle_until(&mut self, completed_cycles: usize) {
        while self.completed_cycles < completed_cycles {
            self.cycle();
        }
    }

    fn take_next_cmd(&mut self) {
        self.current_cmd = self.program.next();
        self.current_cmd_end_cycle =
            self.completed_cycles + self.current_cmd.map(|cmd| cmd.duration()).unwrap_or(0)
    }
}

impl SolutionInput for Vec<Cmd> {
    fn parse(input_str: &str) -> Result<Self> {
        split_parse(input_str, "\n")
    }
}

pub struct Day10Pt1;
impl Solution for Day10Pt1 {
    const DAY: usize = 10;
    const PART: usize = 1;

    type TInput = Vec<Cmd>;
    type TOutput = i32;

    fn solve(_input: &Self::TInput) -> Result<Self::TOutput> {
        let mut vm = VM::new(_input.iter().cloned());
        let mut sum = 0;

        for cycle in (20..=220).step_by(40) {
            vm.cycle_until(cycle);
            sum += vm.signal_strength;
        }

        Ok(sum)
    }
}

pub struct Day10Pt2;
impl Solution for Day10Pt2 {
    const DAY: usize = 10;
    const PART: usize = 1;

    type TInput = Vec<Cmd>;
    type TOutput = Vec<String>;

    fn solve(_input: &Self::TInput) -> Result<Self::TOutput> {
        let mut vm = VM::new(_input.iter().cloned());
        let mut out = Vec::new();

        for _ in 0..6 {
            let mut line = Vec::new();
            for pos in 0..40 {
                let pixel_lit = (-1..=1).contains(&(vm.state.x - pos));
                write!(line, "{}", if pixel_lit { '#' } else { '.' })?;
                vm.cycle();
            }
            out.push(String::from_utf8(line)?);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::util::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<Cmd> = get_input::<Day10Pt1>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<Cmd> = get_input::<Day10Pt1>("input.txt").unwrap();
    }

    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(
            &vec![
                "###...##..###....##..##..###..#..#.###..".to_string(),
                "#..#.#..#.#..#....#.#..#.#..#.#..#.#..#.".to_string(),
                "#..#.#..#.#..#....#.#....###..####.#..#.".to_string(),
                "###..####.###.....#.#....#..#.#..#.###..".to_string(),
                "#....#..#.#....#..#.#..#.#..#.#..#.#....".to_string(),
                "#....#..#.#.....##...##..###..#..#.#....".to_string(),
            ],
            &Day10Pt2::solve(&INPUT_MAIN)?
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            &vec![
                "##..##..##..##..##..##..##..##..##..##..".to_string(),
                "###...###...###...###...###...###...###.".to_string(),
                "####....####....####....####....####....".to_string(),
                "#####.....#####.....#####.....#####.....".to_string(),
                "######......######......######......####".to_string(),
                "#######.......#######.......#######.....".to_string(),
            ],
            &Day10Pt2::solve(&INPUT_TEST)?
        );
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(14340, Day10Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(13140, Day10Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_signal_strength() -> Result<()> {
        let mut vm = VM::new(INPUT_TEST.iter().cloned());

        vm.cycle_until(20);
        assert_eq!(420, vm.signal_strength);

        vm.cycle_until(60);
        assert_eq!(1140, vm.signal_strength);

        vm.cycle_until(100);
        assert_eq!(1800, vm.signal_strength);

        vm.cycle_until(140);
        assert_eq!(2940, vm.signal_strength);

        vm.cycle_until(180);
        assert_eq!(2880, vm.signal_strength);

        vm.cycle_until(220);
        assert_eq!(3960, vm.signal_strength);

        Ok(())
    }

    #[test]
    fn test_vm_cycle() -> Result<()> {
        let program = vec![Cmd::NoOp, Cmd::AddX(3), Cmd::AddX(-5)];
        let mut vm = VM::new(program.iter().cloned());

        assert_eq!(Some(Cmd::NoOp), vm.current_cmd);

        vm.cycle();

        assert_eq!(1, vm.completed_cycles);
        assert_eq!(1, vm.state.x);
        assert_eq!(Some(Cmd::AddX(3)), vm.current_cmd);

        vm.cycle();

        assert_eq!(2, vm.completed_cycles);
        assert_eq!(1, vm.state.x);
        assert_eq!(Some(Cmd::AddX(3)), vm.current_cmd);

        vm.cycle();

        assert_eq!(3, vm.completed_cycles);
        assert_eq!(4, vm.state.x);
        assert_eq!(Some(Cmd::AddX(-5)), vm.current_cmd);

        vm.cycle();

        assert_eq!(4, vm.completed_cycles);
        assert_eq!(4, vm.state.x);
        assert_eq!(Some(Cmd::AddX(-5)), vm.current_cmd);

        vm.cycle();

        assert_eq!(5, vm.completed_cycles);
        assert_eq!(-1, vm.state.x);
        assert_eq!(None, vm.current_cmd);

        vm.cycle();

        assert_eq!(6, vm.completed_cycles);
        assert_eq!(-1, vm.state.x);
        assert_eq!(None, vm.current_cmd);

        Ok(())
    }
}
