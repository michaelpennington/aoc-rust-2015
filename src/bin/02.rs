advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut xs = line.split('x').map(|n| n.parse::<u32>().unwrap());
                let xs = [xs.next().unwrap(), xs.next().unwrap(), xs.next().unwrap()];
                let d = 2 * (xs[0] * xs[1] + xs[1] * xs[2] + xs[0] * xs[2]);
                d + ((xs[0] * xs[1] * xs[2]) / xs[0].max(xs[1].max(xs[2])))
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut xs = line.split('x').map(|n| n.parse::<u32>().unwrap());
                let xs = [xs.next().unwrap(), xs.next().unwrap(), xs.next().unwrap()];
                let d = 2 * (xs[0] + xs[1] + xs[2] - xs[0].max(xs[1].max(xs[2])));
                d + (xs[0] * xs[1] * xs[2])
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(101));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
