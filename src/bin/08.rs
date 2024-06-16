advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_code = 0;
    let mut total_chars = 0;
    for line in input.lines() {
        total_code += line.len() as u32;
        let mut seen_backslash = false;
        let mut in_hex = false;
        for c in line[1..line.len() - 1].chars() {
            if in_hex {
                in_hex = false;
                continue;
            }
            match c {
                '\\' => {
                    if seen_backslash {
                        total_chars += 1;
                    }
                    seen_backslash = !seen_backslash;
                }
                '"' => {
                    assert!(seen_backslash);
                    total_chars += 1;
                    seen_backslash = false;
                }
                'x' if seen_backslash => {
                    in_hex = true;
                    seen_backslash = false;
                }
                _ => total_chars += 1,
            }
        }
    }
    Some(total_code - total_chars)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_code = 0;
    let mut total_chars = 0;
    for line in input.lines() {
        total_code += line.escape_default().count() as u32 + 2;
        total_chars += line.len() as u32;
    }
    Some(total_code - total_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
