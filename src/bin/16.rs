use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

advent_of_code::solution!(16);

#[derive(Debug, Default)]
struct Sue {
    number: u32,
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

#[derive(Debug, Default)]
struct NewSue(Sue);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Categories {
    Children,
    Cats,
    Samoyeds,
    Pomeranians,
    Akitas,
    Vizslas,
    Goldfish,
    Trees,
    Cars,
    Perfumes,
}

impl Index<Categories> for Sue {
    type Output = Option<u32>;

    fn index(&self, index: Categories) -> &Self::Output {
        match index {
            Categories::Children => &self.children,
            Categories::Cats => &self.cats,
            Categories::Samoyeds => &self.samoyeds,
            Categories::Pomeranians => &self.pomeranians,
            Categories::Akitas => &self.akitas,
            Categories::Vizslas => &self.vizslas,
            Categories::Goldfish => &self.goldfish,
            Categories::Trees => &self.trees,
            Categories::Cars => &self.cars,
            Categories::Perfumes => &self.perfumes,
        }
    }
}

impl IndexMut<Categories> for Sue {
    fn index_mut(&mut self, index: Categories) -> &mut Self::Output {
        match index {
            Categories::Children => &mut self.children,
            Categories::Cats => &mut self.cats,
            Categories::Samoyeds => &mut self.samoyeds,
            Categories::Pomeranians => &mut self.pomeranians,
            Categories::Akitas => &mut self.akitas,
            Categories::Vizslas => &mut self.vizslas,
            Categories::Goldfish => &mut self.goldfish,
            Categories::Trees => &mut self.trees,
            Categories::Cars => &mut self.cars,
            Categories::Perfumes => &mut self.perfumes,
        }
    }
}

const CATEGORIES: [Categories; 10] = [
    Categories::Children,
    Categories::Cats,
    Categories::Samoyeds,
    Categories::Pomeranians,
    Categories::Akitas,
    Categories::Vizslas,
    Categories::Goldfish,
    Categories::Trees,
    Categories::Cars,
    Categories::Perfumes,
];

impl FromStr for Categories {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "children" => Ok(Self::Children),
            "cats" => Ok(Self::Cats),
            "samoyeds" => Ok(Self::Samoyeds),
            "pomeranians" => Ok(Self::Pomeranians),
            "akitas" => Ok(Self::Akitas),
            "vizslas" => Ok(Self::Vizslas),
            "goldfish" => Ok(Self::Goldfish),
            "trees" => Ok(Self::Trees),
            "cars" => Ok(Self::Cars),
            "perfumes" => Ok(Self::Perfumes),
            _ => Err(()),
        }
    }
}

impl From<Sue> for NewSue {
    fn from(value: Sue) -> Self {
        Self(value)
    }
}

impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        !CATEGORIES
            .iter()
            .map(|&cat| {
                self[cat]
                    .zip(other[cat])
                    .map(|(item1, item2)| item1 == item2)
            })
            .any(|b| b == Some(false))
    }
}

impl PartialEq for NewSue {
    fn eq(&self, other: &Self) -> bool {
        let sue1 = &self.0;
        let sue2 = &other.0;
        !CATEGORIES
            .iter()
            .map(|&cat| match cat {
                Categories::Cats | Categories::Trees => {
                    sue1[cat].zip(sue2[cat]).map(|(item1, item2)| item1 > item2)
                }
                Categories::Pomeranians | Categories::Goldfish => {
                    sue1[cat].zip(sue2[cat]).map(|(item1, item2)| item1 < item2)
                }
                _ => sue1[cat]
                    .zip(sue2[cat])
                    .map(|(item1, item2)| item1 == item2),
            })
            .any(|b| b == Some(false))
    }
}

impl FromStr for Sue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (beginning, rest) = s.split_once(": ").unwrap();
        let number = beginning.trim_start_matches("Sue ").parse().unwrap();
        let mut out = Self {
            number,
            ..Default::default()
        };
        for (cat, num) in rest.split(", ").map(|s| s.split_once(": ").unwrap()) {
            let cat = cat.parse::<Categories>()?;
            let num = num.parse().unwrap();
            out[cat] = Some(num);
        }
        Ok(out)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let desired_sue = Sue {
        number: 5000,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    for sue in input.lines().map(|l| l.parse::<Sue>().unwrap()) {
        if sue == desired_sue {
            return Some(sue.number);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let desired_sue = NewSue(Sue {
        number: 5000,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    });
    for sue in input
        .lines()
        .map(|l| NewSue::from(l.parse::<Sue>().unwrap()))
    {
        if sue == desired_sue {
            return Some(sue.0.number);
        }
    }
    None
}
