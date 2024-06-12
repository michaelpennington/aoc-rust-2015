advent_of_code::solution!(5);

const BAD_PAIRS: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn is_nice(s: &str) -> bool {
    let mut prev_char = ' ';
    let mut num_vowels = 0;
    let mut two_in_a_row = false;
    for c in s.chars() {
        if BAD_PAIRS.contains(&(prev_char, c)) {
            return false;
        }
        if VOWELS.contains(&c) {
            num_vowels += 1;
        }
        if c == prev_char {
            two_in_a_row = true;
        }
        prev_char = c;
    }
    two_in_a_row && num_vowels >= 3
}

fn is_nicev2(s: &str) -> bool {
    let mut prev_char = ' ';
    let mut scnd_char = ' ';
    let mut pairs = Vec::with_capacity(s.len());
    let mut between = false;
    let mut repeat = false;
    for c in s.chars() {
        if pairs.contains(&(prev_char, c)) {
            repeat = true;
        }
        if prev_char != c && scnd_char == c {
            between = true;
        }
        pairs.push((scnd_char, prev_char));
        scnd_char = prev_char;
        prev_char = c;
        if repeat && between {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| is_nice(l).then_some(()))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| is_nicev2(l).then_some(()))
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }
}
