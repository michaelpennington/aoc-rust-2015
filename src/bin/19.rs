use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
    str::FromStr,
};

advent_of_code::solution!(19);

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseError {
    Element(String),
    // Molecule(String),
    Puzzle(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Element(s) => write!(f, "Element parse error: {}", s),
            // ParseError::Molecule(s) => write!(f, "Molecule parse error: {}", s),
            ParseError::Puzzle(s) => write!(f, "Puzzle parse error: {}", s),
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Element {
    Al,
    Ar,
    B,
    C,
    Ca,
    E,
    F,
    H,
    Mg,
    N,
    O,
    P,
    Rn,
    Si,
    Th,
    Ti,
    Y,
}

impl FromStr for Element {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Al" => Ok(Self::Al),
            "Ar" => Ok(Self::Ar),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "Ca" => Ok(Self::Ca),
            "E" | "e" => Ok(Self::E),
            "F" => Ok(Self::F),
            "H" => Ok(Self::H),
            "Mg" => Ok(Self::Mg),
            "N" => Ok(Self::N),
            "O" => Ok(Self::O),
            "P" => Ok(Self::P),
            "Rn" => Ok(Self::Rn),
            "Si" => Ok(Self::Si),
            "Th" => Ok(Self::Th),
            "Ti" => Ok(Self::Ti),
            "Y" => Ok(Self::Y),
            _ => Err(ParseError::Element(s.into())),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Element::Al => "Al",
            Element::Ar => "Ar",
            Element::B => "B",
            Element::C => "C",
            Element::Ca => "Ca",
            Element::E => "E",
            Element::F => "F",
            Element::H => "H",
            Element::Mg => "Mg",
            Element::N => "N",
            Element::O => "O",
            Element::P => "P",
            Element::Rn => "Rn",
            Element::Si => "Si",
            Element::Th => "Th",
            Element::Ti => "Ti",
            Element::Y => "Y",
        };
        write!(f, "{s}")
    }
}

impl Display for Molecule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.inner {
            write!(f, "{e}")?
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Molecule {
    inner: Vec<Element>,
}

impl Molecule {
    fn get_indices(&self, elem: Element) -> impl Iterator<Item = usize> + '_ {
        self.inner
            .iter()
            .enumerate()
            .filter(move |(_, &e)| e == elem)
            .map(|(i, _)| i)
    }

    fn insert_mol(&mut self, other: Molecule, index: usize) {
        self.inner[index] = *other.inner.first().unwrap();
        for (i, &elem) in other.inner.iter().enumerate().skip(1) {
            self.inner.insert(index + i, elem);
        }
    }

    fn replace_mol(&mut self, start: usize, len: usize, replacement: Element) {
        for _ in start..start + len - 1 {
            self.inner.remove(start);
        }
        self.inner[start] = replacement;
    }

    fn get_indices_mol(&self, mol: Molecule) -> impl Iterator<Item = usize> + '_ {
        self.inner
            .windows(mol.len())
            .enumerate()
            .filter(move |(_, w)| w.iter().zip(mol.inner.iter()).all(|(e1, e2)| e1 == e2))
            .map(|(i, _)| i)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl FromStr for Molecule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inner = Vec::with_capacity(s.len());
        let mut cursor = 0;
        while cursor < s.len() {
            if let Some(c) = s.get(cursor..cursor + 2) {
                if c.chars().nth(1).unwrap().is_ascii_lowercase() {
                    let elem = c.parse()?;
                    inner.push(elem);
                    cursor += 2;
                    continue;
                }
            }
            let elem = s[cursor..cursor + 1].parse()?;
            inner.push(elem);
            cursor += 1;
        }

        Ok(Self { inner })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Puzzle {
    map: HashMap<Element, Vec<Molecule>>,
    molecule: Molecule,
}

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, mol) = s.split_once("\n\n").ok_or(ParseError::Puzzle(s.into()))?;
        let mut map = HashMap::with_capacity(first.lines().count());
        let molecule = mol.trim().parse()?;
        for line in first.lines() {
            let (elem, maps_to) = line
                .split_once(" => ")
                .ok_or(ParseError::Puzzle(s.into()))?;
            let elem = elem.parse()?;
            map.entry(elem)
                .and_modify(|v: &mut Vec<_>| v.push(maps_to.parse().unwrap()))
                .or_insert(vec![maps_to.parse().unwrap()]);
        }
        Ok(Self { map, molecule })
    }
}

impl Puzzle {
    fn calculate(&self) -> usize {
        let mut set: HashSet<Molecule> = HashSet::with_capacity(self.map.len());
        for (&e, maps_to) in &self.map {
            for map in maps_to {
                for i in self.molecule.get_indices(e) {
                    let mut mol = self.molecule.clone();
                    mol.insert_mol(map.clone(), i);
                    set.insert(mol);
                }
            }
        }
        set.len()
    }

    fn calculate_pt2(&self) -> usize {
        let mut set: HashSet<Molecule> = HashSet::new();
        set.insert(self.molecule.clone());
        let goal = Molecule {
            inner: vec![Element::E],
        };
        for index in 1.. {
            let mut new_set: HashSet<Molecule> = HashSet::new();
            for mol in &set {
                for (&e, maps_to) in &self.map {
                    for map in maps_to {
                        for i in mol.get_indices_mol(map.clone()) {
                            let mut mol = mol.clone();
                            // print!("Replacing {map} by {e} at index {i}, {mol} becomes ");
                            mol.replace_mol(i, map.len(), e);
                            // println!("{mol}");
                            if mol == goal {
                                return index;
                            }
                            new_set.insert(mol);
                        }
                    }
                }
            }
            let mut mol_vec = new_set.into_iter().collect::<Vec<_>>();
            mol_vec.sort_by_key(|m| m.len());
            mol_vec.truncate(10);
            set = mol_vec.into_iter().collect();
        }
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let puz = input.parse::<Puzzle>().unwrap();
    Some(puz.calculate() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puz = input.parse::<Puzzle>().unwrap();
    Some(puz.calculate_pt2() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
