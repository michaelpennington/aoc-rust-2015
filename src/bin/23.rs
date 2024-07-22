use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

advent_of_code::solution!(23);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    a: u64,
    b: u64,
    pc: usize,
}

impl Index<Register> for State {
    type Output = u64;

    fn index(&self, index: Register) -> &Self::Output {
        match index {
            Register::A => &self.a,
            Register::B => &self.b,
        }
    }
}

impl IndexMut<Register> for State {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        match index {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
}

impl State {
    fn hlf(&mut self, reg: Register) {
        self[reg] /= 2;
        self.pc += 1;
    }

    fn tpl(&mut self, reg: Register) {
        self[reg] *= 3;
        self.pc += 1;
    }

    fn inc(&mut self, reg: Register) {
        self[reg] += 1;
        self.pc += 1;
    }

    fn jmp(&mut self, offset: isize) {
        self.pc = self.pc.wrapping_add_signed(offset);
    }

    fn jie(&mut self, reg: Register, offset: isize) {
        if self[reg] % 2 == 0 {
            self.pc = self.pc.wrapping_add_signed(offset);
        } else {
            self.pc += 1;
        }
    }

    fn jio(&mut self, reg: Register, offset: isize) {
        if self[reg] == 1 {
            self.pc = self.pc.wrapping_add_signed(offset);
        } else {
            self.pc += 1;
        }
    }

    fn run(&mut self, program: &[Instruction]) -> u64 {
        while let Some(&instruction) = program.get(self.pc) {
            match instruction {
                Instruction::Hlf(reg) => self.hlf(reg),
                Instruction::Tpl(reg) => self.tpl(reg),
                Instruction::Inc(reg) => self.inc(reg),
                Instruction::Jmp(offset) => self.jmp(offset),
                Instruction::Jie(reg, offset) => self.jie(reg, offset),
                Instruction::Jio(reg, offset) => self.jio(reg, offset),
            }
        }
        self.b
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(isize),
    Jie(Register, isize),
    Jio(Register, isize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (inst, rest) = s.split_once(' ').ok_or(())?;
        match inst {
            "hlf" => {
                let reg = rest.parse()?;
                Ok(Self::Hlf(reg))
            }
            "tpl" => {
                let reg = rest.parse()?;
                Ok(Self::Tpl(reg))
            }
            "inc" => {
                let reg = rest.parse()?;
                Ok(Self::Inc(reg))
            }
            "jmp" => {
                let offset = rest.parse().map_err(|_| ())?;
                Ok(Self::Jmp(offset))
            }
            "jie" => {
                let (reg, offset) = rest.split_once(", ").ok_or(())?;
                let reg = reg.parse()?;
                let offset = offset.parse().map_err(|_| ())?;
                Ok(Self::Jie(reg, offset))
            }
            "jio" => {
                let (reg, offset) = rest.split_once(", ").ok_or(())?;
                let reg = reg.parse()?;
                let offset = offset.parse().map_err(|_| ())?;
                Ok(Self::Jio(reg, offset))
            }
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let program: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    let mut state = State::default();
    Some(state.run(&program))
}

pub fn part_two(input: &str) -> Option<u64> {
    let program: Vec<_> = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect();
    let mut state = State {
        a: 1,
        ..State::default()
    };
    Some(state.run(&program))
}
