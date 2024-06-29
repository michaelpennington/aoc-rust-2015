use std::collections::HashMap;

advent_of_code::solution!(20);

const INC: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];

fn prime_factorize(mut n: u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    while div(n, 2) {
        map.entry(2).and_modify(|e| *e += 1).or_insert(1);
        n /= 2;
    }
    while div(n, 3) {
        map.entry(3).and_modify(|e| *e += 1).or_insert(1);
        n /= 3;
    }
    while div(n, 5) {
        map.entry(5).and_modify(|e| *e += 1).or_insert(1);
        n /= 5;
    }
    let mut k = 7;
    let mut i = 0;
    while k * k <= n {
        if div(n, k) {
            map.entry(k).and_modify(|e| *e += 1).or_insert(1);
            n /= k;
        } else {
            k += INC[i];
            i = (i + 1) % 8;
        }
    }
    if n > 1 {
        map.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    map
}

fn div(n: u64, k: u64) -> bool {
    n % k == 0
}

fn f(n: u64) -> u64 {
    let factors = prime_factorize(n);
    factors
        .into_iter()
        .map(|(p, e)| (0..=e).map(|n| p.pow(n as u32)).sum::<u64>())
        .product()
}

fn f2(n: u64) -> u64 {
    let factors = prime_factorize(n);
    let mut all_divisors: Vec<u64> = vec![1];
    for (p, e) in factors {
        let mut new_divisors = Vec::new();
        for i in 0..=e {
            for j in &all_divisors {
                new_divisors.push(p.pow(i as u32) * j);
            }
        }
        all_divisors = new_divisors;
    }
    all_divisors
        .into_iter()
        .filter(|d| n / d <= 50)
        .sum::<u64>()
        * 11
}
pub fn part_one(_input: &str) -> Option<u64> {
    let target = 33_100_000 / 10;
    (1..).find(|&i| f(i) >= target)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let target = 33_100_000;
    (1..).find(|&i| f2(i) >= target)
}
