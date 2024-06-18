use std::collections::{HashMap, HashSet};

advent_of_code::solution!(13);

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

pub fn part_one(input: &str) -> Option<i32> {
    let mut map = HashMap::with_capacity(8);
    let mut names = HashSet::with_capacity(8);
    for line in input.lines() {
        let (name, rest) = line.split_once(" would ")?;
        names.insert(name);
        let (happiness, mut next_to) = rest.split_once(" happiness units by sitting next to ")?;
        next_to = next_to.trim_end_matches('.');
        let hs = happiness.split_whitespace().collect::<Vec<_>>();
        let i: i32 = match hs[0] {
            "gain" => hs[1].parse().ok()?,
            "lose" => -(hs[1].parse().ok()?),
            _ => unreachable!(),
        };
        map.insert((name, next_to), i);
    }
    let names = names.into_iter().collect::<Vec<_>>();
    let perms_iter = PermutationsIter::new(names);
    let mut max_happiness = i32::MIN;
    for mut perm in perms_iter {
        let mut hap = 0;
        perm.push(perm.first()?);
        for names in perm.windows(2) {
            hap += map[&(names[0], names[1])] + map[&(names[1], names[0])];
        }
        max_happiness = max_happiness.max(hap);
    }
    Some(max_happiness)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut map = HashMap::with_capacity(8);
    let mut names = HashSet::with_capacity(8);
    for line in input.lines() {
        let (name, rest) = line.split_once(" would ")?;
        names.insert(name);
        let (happiness, mut next_to) = rest.split_once(" happiness units by sitting next to ")?;
        next_to = next_to.trim_end_matches('.');
        let hs = happiness.split_whitespace().collect::<Vec<_>>();
        let i: i32 = match hs[0] {
            "gain" => hs[1].parse().ok()?,
            "lose" => -(hs[1].parse().ok()?),
            _ => unreachable!(),
        };
        map.insert((name, next_to), i);
    }
    for name in names.iter() {
        map.insert(("Me", name), 0);
        map.insert((name, "Me"), 0);
    }
    names.insert("Me");
    let names = names.into_iter().collect::<Vec<_>>();
    let perms_iter = PermutationsIter::new(names);
    let mut max_happiness = i32::MIN;
    for mut perm in perms_iter {
        let mut hap = 0;
        perm.push(perm.first()?);
        for names in perm.windows(2) {
            hap += map[&(names[0], names[1])] + map[&(names[1], names[0])];
        }
        max_happiness = max_happiness.max(hap);
    }
    Some(max_happiness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }
}
