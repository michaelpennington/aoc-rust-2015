use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Mul},
};

advent_of_code::solution!(17);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Term {
    coeff: u32,
    power: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Polynomial {
    terms: Vec<Term>,
}

fn choose(n: u32, k: u32) -> u32 {
    if k == 0 {
        1
    } else {
        (n * choose(n - 1, k - 1)) / k
    }
}

impl Polynomial {
    fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    fn new_with_mul(num: u32, multiplicity: u32) -> Self {
        let mut terms = Vec::with_capacity(multiplicity as usize);
        for k in 0..=multiplicity {
            terms.push(Term {
                coeff: choose(multiplicity, k),
                power: k * num,
            })
        }
        Self { terms }
    }

    fn coeff_of(&self, power: u32) -> Option<u32> {
        self.terms
            .iter()
            .find_map(|t| (t.power == power).then_some(t.coeff))
    }
}

impl Add<Term> for Polynomial {
    type Output = Polynomial;

    fn add(mut self, rhs: Term) -> Self::Output {
        if self.is_empty() {
            return Polynomial { terms: vec![rhs] };
        }
        if let Some(index) = self
            .terms
            .iter()
            .enumerate()
            .find_map(|(i, t)| (t.power == rhs.power).then_some(i))
        {
            self.terms[index].coeff += rhs.coeff;
        } else if let Some(index) = self
            .terms
            .iter()
            .enumerate()
            .find_map(|(i, t)| (t.power > rhs.power).then_some(i))
        {
            self.terms.insert(index, rhs);
        } else {
            self.terms.push(rhs);
        }
        self
    }
}

impl Add<Polynomial> for Polynomial {
    type Output = Polynomial;
    fn add(mut self, rhs: Polynomial) -> Self::Output {
        if self.is_empty() {
            return rhs;
        }
        for term in rhs.terms {
            self = self + term;
        }
        self
    }
}

impl Mul<Term> for Polynomial {
    type Output = Polynomial;

    fn mul(mut self, rhs: Term) -> Self::Output {
        for term in &mut self.terms {
            term.power += rhs.power;
            term.coeff *= rhs.coeff;
        }
        self
    }
}

impl Mul<Polynomial> for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let mut out = Polynomial {
            terms: Vec::with_capacity(self.terms.len() * rhs.terms.len()),
        };
        for term in rhs.terms {
            out = out + self.clone() * term;
        }
        out
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.power {
            0 => write!(f, "{}", self.coeff),
            n => write!(f, "{}*t^{n}", self.coeff),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.terms.len();
        for term in self.terms.iter().take(len.saturating_sub(1)) {
            write!(f, "{term} + ")?;
        }
        if let Some(term) = self.terms.last() {
            write!(f, "{term}")
        } else {
            Ok(())
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashMap::new();
    for num in input.lines().map(|l| l.parse::<u32>().unwrap()) {
        map.entry(num).and_modify(|i| *i += 1).or_insert(1);
    }
    let mut poly = Polynomial {
        terms: vec![Term { coeff: 1, power: 0 }],
    };
    for (num, mul) in map {
        poly = poly * Polynomial::new_with_mul(num, mul);
    }
    poly.coeff_of(150)
}

fn generate_all_combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    create_all_state(1, n, k, &mut vec![], &mut result);

    result
}

fn create_all_state(
    increment: usize,
    total_number: usize,
    level: usize,
    current_list: &mut Vec<usize>,
    total_list: &mut Vec<Vec<usize>>,
) {
    if level == 0 {
        total_list.push(current_list.clone());
        return;
    }

    for i in increment..(total_number + 2 - level) {
        current_list.push(i - 1);
        create_all_state(i + 1, total_number, level - 1, current_list, total_list);
        current_list.pop();
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut count = 0;
    for comb in generate_all_combinations(20, 4) {
        if comb.iter().map(|&i| nums[i]).sum::<u32>() == 150 {
            count += 1;
        }
    }
    Some(count)
}
