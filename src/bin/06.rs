use std::str::FromStr;

advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    from: (usize, usize),
    to: (usize, usize),
    cmd: Command,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cmd, rest) = s.split_at(s.find(char::is_numeric).ok_or(())?);
        let cmd: Command = cmd.parse()?;
        let mut coords = rest
            .split(" through ")
            .flat_map(|r| r.split(',').map(|r| r.parse().unwrap()));
        let (from, to) = (
            (coords.next().unwrap(), coords.next().unwrap()),
            (coords.next().unwrap(), coords.next().unwrap()),
        );
        Ok(Self { from, to, cmd })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" | "turn on " => Ok(Self::TurnOn),
            "toggle" | "toggle " => Ok(Self::Toggle),
            "turn off" | "turn off " => Ok(Self::TurnOff),
            _ => Err(()),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lights = [[false; 1000]; 1000];
    for inst in input.lines().map(|l| l.parse::<Instruction>().unwrap()) {
        let f = match inst.cmd {
            Command::TurnOn => |_: bool| true,
            Command::TurnOff => |_: bool| false,
            Command::Toggle => |b: bool| !b,
        };
        for row in lights.iter_mut().take(inst.to.0 + 1).skip(inst.from.0) {
            for item in row.iter_mut().take(inst.to.1 + 1).skip(inst.from.1) {
                *item = f(*item)
            }
        }
    }
    Some(
        lights
            .iter()
            .flat_map(|l| l.iter().map(|&b| if b { 1 } else { 0 }))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lights = [[0u32; 1000]; 1000];
    for inst in input.lines().map(|l| l.parse::<Instruction>().unwrap()) {
        let f = match inst.cmd {
            Command::TurnOn => |i: u32| i + 1,
            Command::TurnOff => |i: u32| i.saturating_sub(1),
            Command::Toggle => |i: u32| i + 2,
        };
        for row in lights.iter_mut().take(inst.to.0 + 1).skip(inst.from.0) {
            for item in row.iter_mut().take(inst.to.1 + 1).skip(inst.from.1) {
                *item = f(*item)
            }
        }
    }
    Some(
        lights
            .iter()
            .flat_map(|l| l.iter())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1_000_000));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(999_000));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(998_996));
    }
}
