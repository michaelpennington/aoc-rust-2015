advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input.chars().filter(|&c| c == '(').count() as i32
            - input.chars().filter(|&c| c == ')').count() as i32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut count = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => count += 1,
            ')' => count -= 1,
            _ => unreachable!(),
        }
        if count == -1 {
            return Some(i as u32 + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_five() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_six() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_part_one_seven() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_part_one_eight() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 8,
        ));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_one_nine() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 9,
        ));
        assert_eq!(result, Some(-3));
    }

    #[test]
    fn test_part_two_ten() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 10,
        ));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two_eleven() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 11,
        ));
        assert_eq!(result, Some(5));
    }
}
