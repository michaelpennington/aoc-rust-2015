use std::str::FromStr;

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Sequence {
    digits: Vec<u8>,
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .trim()
            .chars()
            .map(|c| match c {
                '1' => 1,
                '2' => 2,
                '3' => 3,
                _ => unreachable!(),
            })
            .collect();
        Ok(Self { digits })
    }
}

impl Sequence {
    fn next(&self) -> Self {
        let mut current_digit = *self.digits.first().unwrap();
        let mut counter = 0;
        let mut digits = Vec::with_capacity(self.digits.len() * 2);
        for &digit in &self.digits {
            if digit == current_digit {
                counter += 1;
            } else {
                digits.push(counter);
                digits.push(current_digit);
                counter = 1;
            }
            current_digit = digit;
        }
        digits.push(counter);
        digits.push(current_digit);
        Self { digits }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut seq = input.parse::<Sequence>().ok()?;
    for _ in 0..40 {
        seq = seq.next();
    }
    Some(seq.digits.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut seq = input.parse::<Sequence>().ok()?;
    for _ in 0..50 {
        seq = seq.next();
    }
    Some(seq.digits.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
