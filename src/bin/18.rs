use std::{
    convert::identity,
    ops::{Index, IndexMut},
    str::FromStr,
};

advent_of_code::solution!(18);

#[derive(Debug, PartialEq, Eq, Clone)]
struct GameOfLife {
    cells: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl GameOfLife {
    fn step(&mut self) {
        let mut new_map = vec![vec![false; self.width]; self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                match (self[(x, y)], self.num_neighbors((x, y))) {
                    (true, 2..=3) => {
                        new_map[y][x] = true;
                    }
                    (false, 3) => {
                        new_map[y][x] = true;
                    }
                    _ => {}
                }
            }
        }
        self.cells = new_map;
    }

    fn step_special(&mut self) {
        let mut new_map = vec![vec![false; self.width]; self.height];
        new_map[0][0] = true;
        new_map[self.height - 1][0] = true;
        new_map[0][self.width - 1] = true;
        new_map[self.height - 1][self.width - 1] = true;
        for x in 0..self.width {
            for y in 0..self.height {
                if (x, y) == (0, 0)
                    || (x, y) == (self.width - 1, 0)
                    || (x, y) == (0, self.height - 1)
                    || (x, y) == (self.width - 1, self.height - 1)
                {
                    continue;
                }
                match (self[(x, y)], self.num_neighbors((x, y))) {
                    (true, 2..=3) => {
                        new_map[y][x] = true;
                    }
                    (false, 3) => {
                        new_map[y][x] = true;
                    }
                    _ => {}
                }
            }
        }
        self.cells = new_map;
    }

    fn num_neighbors(&self, pt: (usize, usize)) -> usize {
        (pt.0.saturating_sub(1)..(pt.0 + 2).min(self.width))
            .flat_map(|x| {
                (pt.1.saturating_sub(1)..(pt.1 + 2).min(self.height)).map(move |y| (x, y))
            })
            .filter(|&point| point != pt)
            .map(|pt| self[pt])
            .filter(|&b| identity(b))
            .count()
    }

    fn num_on(&self) -> usize {
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| self[(x, y)]))
            .filter(|&b| identity(b))
            .count()
    }
}

impl Index<(usize, usize)> for GameOfLife {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.width && index.1 < self.height);
        &self.cells[index.1][index.0]
    }
}

impl IndexMut<(usize, usize)> for GameOfLife {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.width && index.1 < self.height);
        &mut self.cells[index.1][index.0]
    }
}

impl FromStr for GameOfLife {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Unaccounted for char {c}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let width = cells[0].len();
        let height = cells.len();
        for line in &cells {
            assert!(line.len() == width);
        }
        Ok(Self {
            cells,
            width,
            height,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut gol = input.parse::<GameOfLife>().ok()?;
    for _ in 0..100 {
        gol.step();
    }
    Some(gol.num_on() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut gol = input.parse::<GameOfLife>().ok()?;
    gol.cells[0][0] = true;
    gol.cells[gol.height - 1][0] = true;
    gol.cells[0][gol.width - 1] = true;
    gol.cells[gol.height - 1][gol.width - 1] = true;
    for _ in 0..100 {
        gol.step_special();
    }
    Some(gol.num_on() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
