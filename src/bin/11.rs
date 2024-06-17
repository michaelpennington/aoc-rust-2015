use std::{fmt::Display, str::FromStr};

advent_of_code::solution!(11);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Password([AsciiChar; 8]);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AsciiChar {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl AsciiChar {
    const fn next(self) -> (Self, bool) {
        match self {
            AsciiChar::A => (AsciiChar::B, false),
            AsciiChar::B => (AsciiChar::C, false),
            AsciiChar::C => (AsciiChar::D, false),
            AsciiChar::D => (AsciiChar::E, false),
            AsciiChar::E => (AsciiChar::F, false),
            AsciiChar::F => (AsciiChar::G, false),
            AsciiChar::G => (AsciiChar::H, false),
            AsciiChar::H => (AsciiChar::I, false),
            AsciiChar::I => (AsciiChar::J, false),
            AsciiChar::J => (AsciiChar::K, false),
            AsciiChar::K => (AsciiChar::L, false),
            AsciiChar::L => (AsciiChar::M, false),
            AsciiChar::M => (AsciiChar::N, false),
            AsciiChar::N => (AsciiChar::O, false),
            AsciiChar::O => (AsciiChar::P, false),
            AsciiChar::P => (AsciiChar::Q, false),
            AsciiChar::Q => (AsciiChar::R, false),
            AsciiChar::R => (AsciiChar::S, false),
            AsciiChar::S => (AsciiChar::T, false),
            AsciiChar::T => (AsciiChar::U, false),
            AsciiChar::U => (AsciiChar::V, false),
            AsciiChar::V => (AsciiChar::W, false),
            AsciiChar::W => (AsciiChar::X, false),
            AsciiChar::X => (AsciiChar::Y, false),
            AsciiChar::Y => (AsciiChar::Z, false),
            AsciiChar::Z => (AsciiChar::A, true),
        }
    }
}

impl TryFrom<char> for AsciiChar {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' | 'A' => Ok(Self::A),
            'b' | 'B' => Ok(Self::B),
            'c' | 'C' => Ok(Self::C),
            'd' | 'D' => Ok(Self::D),
            'e' | 'E' => Ok(Self::E),
            'f' | 'F' => Ok(Self::F),
            'g' | 'G' => Ok(Self::G),
            'h' | 'H' => Ok(Self::H),
            'i' | 'I' => Ok(Self::I),
            'j' | 'J' => Ok(Self::J),
            'k' | 'K' => Ok(Self::K),
            'l' | 'L' => Ok(Self::L),
            'm' | 'M' => Ok(Self::M),
            'n' | 'N' => Ok(Self::N),
            'o' | 'O' => Ok(Self::O),
            'p' | 'P' => Ok(Self::P),
            'q' | 'Q' => Ok(Self::Q),
            'r' | 'R' => Ok(Self::R),
            's' | 'S' => Ok(Self::S),
            't' | 'T' => Ok(Self::T),
            'u' | 'U' => Ok(Self::U),
            'v' | 'V' => Ok(Self::V),
            'w' | 'W' => Ok(Self::W),
            'x' | 'X' => Ok(Self::X),
            'y' | 'Y' => Ok(Self::Y),
            'z' | 'Z' => Ok(Self::Z),
            _ => Err(()),
        }
    }
}

impl From<&AsciiChar> for char {
    fn from(value: &AsciiChar) -> Self {
        match value {
            AsciiChar::A => 'a',
            AsciiChar::B => 'b',
            AsciiChar::C => 'c',
            AsciiChar::D => 'd',
            AsciiChar::E => 'e',
            AsciiChar::F => 'f',
            AsciiChar::G => 'g',
            AsciiChar::H => 'h',
            AsciiChar::I => 'i',
            AsciiChar::J => 'j',
            AsciiChar::K => 'k',
            AsciiChar::L => 'l',
            AsciiChar::M => 'm',
            AsciiChar::N => 'n',
            AsciiChar::O => 'o',
            AsciiChar::P => 'p',
            AsciiChar::Q => 'q',
            AsciiChar::R => 'r',
            AsciiChar::S => 's',
            AsciiChar::T => 't',
            AsciiChar::U => 'u',
            AsciiChar::V => 'v',
            AsciiChar::W => 'w',
            AsciiChar::X => 'x',
            AsciiChar::Y => 'y',
            AsciiChar::Z => 'z',
        }
    }
}

impl Display for AsciiChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = self.into();
        write!(f, "{c}")
    }
}

impl FromStr for Password {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = [AsciiChar::A; 8];
        for (i, c) in s.chars().enumerate() {
            out[i] = c.try_into()?;
        }
        Ok(Self(out))
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl Password {
    fn next(&mut self) {
        let len = self.0.len();
        for i in (1..=len).map(|i| len - i) {
            let (next, carry) = self.0[i].next();
            self.0[i] = next;
            if !carry {
                break;
            }
        }
        if let Some(pos) = self
            .0
            .iter()
            .position(|&c| c == AsciiChar::I || c == AsciiChar::O || c == AsciiChar::L)
        {
            self.0[pos] = self.0[pos].next().0;
            for i in pos + 1..len {
                self.0[i] = AsciiChar::A;
            }
        }
    }

    fn is_valid(&self) -> bool {
        let mut consecutive = false;
        let mut doubles = 0;
        for win in self.0.windows(3) {
            if win[0].next() == (win[1], false) && win[1].next() == (win[2], false) {
                consecutive = true;
            }
        }
        let mut saw_double = false;
        for slice in self.0.windows(2) {
            if saw_double {
                saw_double = false;
                continue;
            }
            if slice[0] == slice[1] {
                saw_double = true;
                doubles += 1;
            }
        }
        consecutive && doubles >= 2
    }
}

pub fn part_one(input: &str) -> Option<Password> {
    let mut password: Password = input.trim().parse().ok()?;
    while !password.is_valid() {
        password.next();
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<Password> {
    let mut password: Password = input.trim().parse().ok()?;
    while !password.is_valid() {
        password.next();
    }
    password.next();
    while !password.is_valid() {
        password.next();
    }
    Some(password)
}
