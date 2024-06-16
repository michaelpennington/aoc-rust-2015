use std::collections::{HashMap, HashSet};

advent_of_code::solution!(9);

#[derive(Debug)]
struct PermutationsIter<T> {
    items: Vec<T>,
    state: Vec<usize>,
    i: usize,
}

impl<T> PermutationsIter<T> {
    fn new(items: Vec<T>) -> Self {
        let state = vec![0; items.len()];
        let i = 1;
        Self { items, state, i }
    }
}

impl<T> Iterator for PermutationsIter<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.i < self.items.len() {
                if self.state[self.i] < self.i {
                    if self.i % 2 == 0 {
                        self.items.swap(0, self.i);
                    } else {
                        self.items.swap(self.state[self.i], self.i);
                    }
                    self.state[self.i] += 1;
                    self.i = 1;
                    break Some(self.items.clone());
                } else {
                    self.state[self.i] = 0;
                    self.i += 1;
                }
            } else {
                break None;
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::with_capacity(8);
    let mut locations = HashSet::with_capacity(8);
    for line in input.lines() {
        let (places, dist) = line.split_once(" = ")?;
        let dist = dist.parse::<u32>().ok()?;
        let (from, to) = places.split_once(" to ")?;
        map.insert((from, to), dist);
        map.insert((to, from), dist);
        locations.insert(from);
        locations.insert(to);
    }
    let locations = locations.into_iter().collect::<Vec<_>>();
    let mut ans = u32::MAX;
    for permutation in PermutationsIter::new(locations) {
        let mut total = 0;
        for pair in permutation.windows(2) {
            total += map[&(pair[0], pair[1])];
        }
        ans = ans.min(total);
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = HashMap::with_capacity(8);
    let mut locations = HashSet::with_capacity(8);
    for line in input.lines() {
        let (places, dist) = line.split_once(" = ")?;
        let dist = dist.parse::<u32>().ok()?;
        let (from, to) = places.split_once(" to ")?;
        map.insert((from, to), dist);
        map.insert((to, from), dist);
        locations.insert(from);
        locations.insert(to);
    }
    let locations = locations.into_iter().collect::<Vec<_>>();
    let mut ans = u32::MIN;
    for permutation in PermutationsIter::new(locations) {
        let mut total = 0;
        for pair in permutation.windows(2) {
            total += map[&(pair[0], pair[1])];
        }
        ans = ans.max(total);
    }
    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(605));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(982));
    }
}
