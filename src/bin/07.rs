use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(7);

enum Op {
    And,
    Or,
    Not,
    LShift,
    RShift,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "NOT" => Ok(Self::Not),
            "LSHIFT" => Ok(Self::LShift),
            "RSHIFT" => Ok(Self::RShift),
            _ => Err(()),
        }
    }
}

fn is_number(s: &str) -> Option<u16> {
    s.chars()
        .all(|c| c.is_numeric())
        .then(|| s.parse().ok())
        .flatten()
}

pub fn part_one(input: &str) -> Option<u16> {
    let mut dict: HashMap<&str, u16> = HashMap::new();
    let mut lines = input
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .collect::<Vec<_>>();
    let mut holder: Vec<(&str, &str)> = Vec::new();
    while !lines.is_empty() {
        for (inst, dest) in &lines {
            let instructions = inst.split_whitespace().collect::<Vec<_>>();
            match instructions.len() {
                1 => {
                    if let Some(n) = is_number(instructions[0]) {
                        dict.insert(dest, n);
                    } else if let Some(&n) = dict.get(instructions[0]) {
                        dict.insert(dest, n);
                    } else {
                        holder.push((inst, dest));
                    }
                }
                2 => {
                    if let Some(n) = is_number(instructions[1]) {
                        dict.insert(dest, !n);
                    } else if let Some(&n) = dict.get(instructions[1]) {
                        dict.insert(dest, !n);
                    } else {
                        holder.push((inst, dest));
                    }
                }
                3 => {
                    if let Some((n, m)) = is_number(instructions[0])
                        .zip(is_number(instructions[2]))
                        .or_else(|| {
                            is_number(instructions[0]).zip(dict.get(instructions[2]).copied())
                        })
                        .or_else(|| {
                            dict.get(instructions[0])
                                .copied()
                                .zip(is_number(instructions[2]))
                        })
                        .or_else(|| {
                            dict.get(instructions[0])
                                .copied()
                                .zip(dict.get(instructions[2]).copied())
                        })
                    {
                        match instructions[1].parse::<Op>().unwrap() {
                            Op::And => dict.insert(dest, n & m),
                            Op::Or => dict.insert(dest, n | m),
                            Op::LShift => dict.insert(dest, n << m),
                            Op::RShift => dict.insert(dest, n >> m),
                            _ => unreachable!(),
                        };
                    } else {
                        holder.push((inst, dest));
                    }
                }

                _ => unreachable!(),
            }
        }
        lines.clone_from(&holder);
        holder = Vec::new();
    }
    dict.get("a").copied()
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut dict: HashMap<&str, u16> = HashMap::new();
    let mut lines = input
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .collect::<Vec<_>>();
    let mut holder: Vec<(&str, &str)> = Vec::new();
    while !lines.is_empty() {
        for (inst, dest) in &lines {
            let instructions = inst.split_whitespace().collect::<Vec<_>>();
            match instructions.len() {
                1 => {
                    if let Some(n) = is_number(instructions[0]) {
                        dict.insert(dest, n);
                    } else if let Some(&n) = dict.get(instructions[0]) {
                        dict.insert(dest, n);
                    } else {
                        holder.push((inst, dest));
                    }
                }
                2 => {
                    if let Some(n) = is_number(instructions[1]) {
                        dict.insert(dest, !n);
                    } else if let Some(&n) = dict.get(instructions[1]) {
                        dict.insert(dest, !n);
                    } else {
                        holder.push((inst, dest));
                    }
                }
                3 => {
                    if let Some((n, m)) = is_number(instructions[0])
                        .zip(is_number(instructions[2]))
                        .or_else(|| {
                            is_number(instructions[0]).zip(dict.get(instructions[2]).copied())
                        })
                        .or_else(|| {
                            dict.get(instructions[0])
                                .copied()
                                .zip(is_number(instructions[2]))
                        })
                        .or_else(|| {
                            dict.get(instructions[0])
                                .copied()
                                .zip(dict.get(instructions[2]).copied())
                        })
                    {
                        match instructions[1].parse::<Op>().unwrap() {
                            Op::And => dict.insert(dest, n & m),
                            Op::Or => dict.insert(dest, n | m),
                            Op::LShift => dict.insert(dest, n << m),
                            Op::RShift => dict.insert(dest, n >> m),
                            _ => unreachable!(),
                        };
                    } else {
                        holder.push((inst, dest));
                    }
                }

                _ => unreachable!(),
            }
        }
        lines.clone_from(&holder);
        holder = Vec::new();
    }
    let &b = dict.get("a").unwrap();
    let mut dict: HashMap<&str, u16> = HashMap::new();
    dict.insert("b", b);
    let mut lines = input
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .collect::<Vec<_>>();
    let mut holder: Vec<(&str, &str)> = Vec::new();
    while !lines.is_empty() {
        for (inst, dest) in &lines {
            let instructions = inst.split_whitespace().collect::<Vec<_>>();
            match instructions.len() {
                1 => {
                    if dest != &"b" {
                        if let Some(n) = is_number(instructions[0]) {
                            dict.insert(dest, n);
                        } else if let Some(&n) = dict.get(instructions[0]) {
                            dict.insert(dest, n);
                        } else {
                            holder.push((inst, dest));
                        }
                    }
                }
                2 => {
                    if let Some(n) = is_number(instructions[1]) {
                        dict.insert(dest, !n);
                    } else if let Some(&n) = dict.get(instructions[1]) {
                        dict.insert(dest, !n);
                    } else {
                        holder.push((inst, dest));
                    }
                }
                3 => {
                    if let Some((n, m)) = is_number(instructions[0])
                        .zip(is_number(instructions[2]))
                        .or_else(|| {
                            is_number(instructions[0]).zip(dict.get(instructions[2]).copied())
                        })
                        .or_else(|| {
                            dict.get(instructions[0])
                                .copied()
                                .zip(is_number(instructions[2]))
                        })
                        .or_else(|| {
                            dict.get(instructions[0])
                                .copied()
                                .zip(dict.get(instructions[2]).copied())
                        })
                    {
                        match instructions[1].parse::<Op>().unwrap() {
                            Op::And => dict.insert(dest, n & m),
                            Op::Or => dict.insert(dest, n | m),
                            Op::LShift => dict.insert(dest, n << m),
                            Op::RShift => dict.insert(dest, n >> m),
                            _ => unreachable!(),
                        };
                    } else {
                        holder.push((inst, dest));
                    }
                }

                _ => unreachable!(),
            }
        }
        lines.clone_from(&holder);
        holder = Vec::new();
    }
    dict.get("a").copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(507));
    }
}
