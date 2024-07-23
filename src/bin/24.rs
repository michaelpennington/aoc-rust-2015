use itertools::Itertools;

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let nums = input.lines().map(|l| l.parse::<u64>().unwrap());
    let mut smallest = u64::MAX;
    for comb in nums
        .combinations(6)
        .filter(|comb| comb.iter().sum::<u64>() == 512)
    {
        smallest = smallest.min(comb.iter().product());
    }
    Some(smallest)
}

pub fn part_two(input: &str) -> Option<u64> {
    let nums = input.lines().map(|l| l.parse::<u64>().unwrap());
    let mut smallest = u64::MAX;
    for comb in nums
        .combinations(4)
        .filter(|comb| comb.iter().sum::<u64>() == 384)
    {
        smallest = smallest.min(comb.iter().product());
    }
    Some(smallest)
}
