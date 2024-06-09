use std::collections::HashSet;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0, 0);
    let mut set = HashSet::new();
    set.insert(pos);
    for c in input.trim().chars() {
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => unreachable!(),
        }
        set.insert(pos);
    }
    Some(set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    let mut set = HashSet::new();
    set.insert(pos1);
    let mut p1 = true;
    for c in input.trim().chars() {
        let pos = if p1 { &mut pos1 } else { &mut pos2 };
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => unreachable!(),
        }
        set.insert(*pos);
        p1 = !p1;
    }
    Some(set.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(11));
    }
}
