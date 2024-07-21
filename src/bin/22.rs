use std::ops::{Index, IndexMut};

advent_of_code::solution!(22);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
enum Spell {
    MagicMissile = 53,
    Drain = 73,
    Shield = 113,
    Poison = 173,
    Recharge = 229,
}

impl Spell {
    fn nth(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::MagicMissile),
            1 => Some(Self::Drain),
            2 => Some(Self::Shield),
            3 => Some(Self::Poison),
            4 => Some(Self::Recharge),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct ActiveEffects([u8; 5]);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct ActiveEffectsIter {
    effects: ActiveEffects,
    index: usize,
}

impl IntoIterator for ActiveEffects {
    type Item = (Spell, u8);

    type IntoIter = ActiveEffectsIter;

    fn into_iter(self) -> Self::IntoIter {
        ActiveEffectsIter {
            effects: self,
            index: 0,
        }
    }
}

impl Iterator for ActiveEffectsIter {
    type Item = (Spell, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let spell = Spell::nth(self.index)?;
        self.index += 1;
        Some((spell, self.effects[spell]))
    }
}

impl Index<Spell> for ActiveEffects {
    type Output = u8;

    fn index(&self, index: Spell) -> &Self::Output {
        match index {
            Spell::MagicMissile => &self.0[0],
            Spell::Drain => &self.0[1],
            Spell::Shield => &self.0[2],
            Spell::Poison => &self.0[3],
            Spell::Recharge => &self.0[4],
        }
    }
}

impl IndexMut<Spell> for ActiveEffects {
    fn index_mut(&mut self, index: Spell) -> &mut Self::Output {
        match index {
            Spell::MagicMissile => &mut self.0[0],
            Spell::Drain => &mut self.0[1],
            Spell::Shield => &mut self.0[2],
            Spell::Poison => &mut self.0[3],
            Spell::Recharge => &mut self.0[4],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    mana: u32,
    health: u32,
    boss_health: u32,
    total_mana_used: u32,
    active_effects: ActiveEffects,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mana: 500,
            health: 50,
            boss_health: 51,
            total_mana_used: 0,
            active_effects: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TriOption {
    Win,
    Lose,
    Continue,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameOption {
    Win(u32),
    Lose,
    Error,
    NotDone,
}

impl State {
    fn process(&mut self, seq: &[Spell]) -> GameOption {
        for spell in seq {
            match self.cast(*spell) {
                TriOption::Win => return GameOption::Win(self.total_mana_used),
                TriOption::Lose => return GameOption::Lose,
                TriOption::Error => return GameOption::Error,
                _ => {}
            }
        }
        GameOption::NotDone
    }

    fn cast(&mut self, spell: Spell) -> TriOption {
        // apply effects
        for (spell, turns) in self.active_effects {
            if turns == 0 {
                continue;
            }
            match spell {
                Spell::MagicMissile => unreachable!(),
                Spell::Drain => unreachable!(),
                Spell::Shield => {
                    self.active_effects[spell] -= 1;
                }
                Spell::Poison => {
                    if let Some(bh) = self.boss_health.checked_sub(3) {
                        self.boss_health = bh;
                    } else {
                        return TriOption::Win;
                    }
                    self.active_effects[spell] -= 1;
                }
                Spell::Recharge => {
                    self.mana += 101;
                    self.active_effects[spell] -= 1;
                }
            }
        }
        // cast spell
        if let Some(remaining_mana) = self.mana.checked_sub(spell as u32) {
            self.mana = remaining_mana;
            self.total_mana_used += spell as u32;
        } else {
            return TriOption::Lose;
        }
        match spell {
            Spell::MagicMissile => {
                if let Some(bh) = self.boss_health.checked_sub(4) {
                    self.boss_health = bh;
                } else {
                    return TriOption::Win;
                }
            }
            Spell::Drain => {
                if let Some(bh) = self.boss_health.checked_sub(2) {
                    self.boss_health = bh
                } else {
                    return TriOption::Win;
                };
                self.health += 2;
            }
            Spell::Shield => {
                if self.active_effects[spell] != 0 {
                    return TriOption::Error;
                }
                self.active_effects[spell] = 6
            }
            Spell::Poison => {
                if self.active_effects[spell] != 0 {
                    return TriOption::Error;
                }
                self.active_effects[spell] = 6
            }
            Spell::Recharge => {
                if self.active_effects[spell] != 0 {
                    return TriOption::Error;
                }
                self.active_effects[spell] = 5
            }
        }
        // boss turn
        for (spell, turns) in self.active_effects {
            if turns == 0 {
                continue;
            }
            match spell {
                Spell::MagicMissile => {
                    unreachable!()
                }
                Spell::Drain => {
                    unreachable!()
                }
                Spell::Shield => self.active_effects[spell] -= 1,
                Spell::Poison => {
                    if let Some(bh) = self.boss_health.checked_sub(3) {
                        self.boss_health = bh;
                    } else {
                        return TriOption::Win;
                    }
                    self.active_effects[spell] -= 1;
                }
                Spell::Recharge => {
                    self.mana += 101;
                    self.active_effects[spell] -= 1;
                }
            }
        }
        if self.active_effects[Spell::Shield] > 0 {
            if let Some(ph) = self.health.checked_sub(2) {
                self.health = ph;
            } else {
                return TriOption::Lose;
            }
        } else if let Some(ph) = self.health.checked_sub(9) {
            self.health = ph;
        } else {
            return TriOption::Lose;
        }
        TriOption::Continue
    }

    fn process_hard(&mut self, seq: &[Spell]) -> GameOption {
        for spell in seq {
            match self.cast_hard(*spell) {
                TriOption::Win => return GameOption::Win(self.total_mana_used),
                TriOption::Lose => return GameOption::Lose,
                TriOption::Error => return GameOption::Error,
                _ => {}
            }
        }
        GameOption::NotDone
    }

    fn cast_hard(&mut self, spell: Spell) -> TriOption {
        // apply effects
        if let Some(ph) = self.health.checked_sub(1) {
            if ph != 0 {
                self.health = ph;
            } else {
                return TriOption::Lose;
            }
        } else {
            return TriOption::Lose;
        }
        for (spell, turns) in self.active_effects {
            if turns == 0 {
                continue;
            }
            match spell {
                Spell::MagicMissile => unreachable!(),
                Spell::Drain => unreachable!(),
                Spell::Shield => {
                    self.active_effects[spell] -= 1;
                }
                Spell::Poison => {
                    if let Some(bh) = self.boss_health.checked_sub(3) {
                        if bh == 0 {
                            return TriOption::Win;
                        } else {
                            self.boss_health = bh;
                        }
                    } else {
                        return TriOption::Win;
                    }
                    self.active_effects[spell] -= 1;
                }
                Spell::Recharge => {
                    self.mana += 101;
                    self.active_effects[spell] -= 1;
                }
            }
        }
        // cast spell
        if let Some(remaining_mana) = self.mana.checked_sub(spell as u32) {
            self.mana = remaining_mana;
            self.total_mana_used += spell as u32;
        } else {
            return TriOption::Lose;
        }
        match spell {
            Spell::MagicMissile => {
                if let Some(bh) = self.boss_health.checked_sub(4) {
                    if bh == 0 {
                        return TriOption::Win;
                    } else {
                        self.boss_health = bh;
                    }
                } else {
                    return TriOption::Win;
                }
            }
            Spell::Drain => {
                if let Some(bh) = self.boss_health.checked_sub(2) {
                    if bh == 0 {
                        return TriOption::Win;
                    } else {
                        self.boss_health = bh;
                    }
                } else {
                    return TriOption::Win;
                };
                self.health += 2;
            }
            Spell::Shield => {
                if self.active_effects[spell] != 0 {
                    return TriOption::Error;
                }
                self.active_effects[spell] = 6
            }
            Spell::Poison => {
                if self.active_effects[spell] != 0 {
                    return TriOption::Error;
                }
                self.active_effects[spell] = 6
            }
            Spell::Recharge => {
                if self.active_effects[spell] != 0 {
                    return TriOption::Error;
                }
                self.active_effects[spell] = 5
            }
        }
        // boss turn
        for (spell, turns) in self.active_effects {
            if turns == 0 {
                continue;
            }
            match spell {
                Spell::MagicMissile => {
                    unreachable!()
                }
                Spell::Drain => {
                    unreachable!()
                }
                Spell::Shield => self.active_effects[spell] -= 1,
                Spell::Poison => {
                    if let Some(bh) = self.boss_health.checked_sub(3) {
                        if bh == 0 {
                            return TriOption::Win;
                        } else {
                            self.boss_health = bh;
                        }
                    } else {
                        return TriOption::Win;
                    }
                    self.active_effects[spell] -= 1;
                }
                Spell::Recharge => {
                    self.mana += 101;
                    self.active_effects[spell] -= 1;
                }
            }
        }
        if self.active_effects[Spell::Shield] > 0 {
            if let Some(ph) = self.health.checked_sub(2) {
                if ph != 0 {
                    self.health = ph;
                } else {
                    return TriOption::Lose;
                }
            } else {
                return TriOption::Lose;
            }
        } else if let Some(ph) = self.health.checked_sub(9) {
            if ph != 0 {
                self.health = ph;
            } else {
                return TriOption::Lose;
            }
        } else {
            return TriOption::Lose;
        }
        TriOption::Continue
    }
}

fn try_permutations(choices: &mut Vec<(Vec<Spell>, State)>) -> Vec<u32> {
    let mut out = Vec::new();
    let mut new_spells = Vec::new();
    for spells in &*choices {
        let mut state = State::default();
        match state.process(&spells.0) {
            GameOption::Win(win) => {
                out.push(win);
            }
            GameOption::NotDone => {
                for spell in [
                    Spell::MagicMissile,
                    Spell::Drain,
                    Spell::Shield,
                    Spell::Poison,
                    Spell::Recharge,
                ] {
                    let mut spells = spells.0.clone();
                    spells.push(spell);
                    new_spells.push((spells, state));
                }
            }
            _ => {}
        }
    }
    choices.clear();
    choices.extend(new_spells);
    choices.sort_by_key(|s| {
        (s.1.total_mana_used + s.1.boss_health * 100).saturating_sub(s.1.health * 100)
    });
    choices.truncate(500);
    out
}

fn try_permutations_hard(choices: &mut Vec<(Vec<Spell>, State)>) -> Vec<(Vec<Spell>, u32)> {
    let mut out = Vec::new();
    let mut new_spells = Vec::new();
    for spells in &*choices {
        let mut state = State::default();
        match state.process_hard(&spells.0) {
            GameOption::Win(win) => {
                out.push((spells.0.clone(), win));
            }
            GameOption::NotDone => {
                for spell in [
                    Spell::MagicMissile,
                    Spell::Drain,
                    Spell::Shield,
                    Spell::Poison,
                    Spell::Recharge,
                ] {
                    let mut spells = spells.0.clone();
                    spells.push(spell);
                    new_spells.push((spells, state));
                }
            }
            _ => {}
        }
    }
    choices.clear();
    choices.extend(new_spells);
    choices.sort_by_key(|s| {
        (s.1.total_mana_used + s.1.boss_health * 100).saturating_sub(s.1.health * 100)
    });
    choices.truncate(4000);
    out
}

pub fn part_one(_input: &str) -> Option<u32> {
    let mut spells = vec![
        (vec![Spell::MagicMissile], State::default()),
        (vec![Spell::Drain], State::default()),
        (vec![Spell::Shield], State::default()),
        (vec![Spell::Poison], State::default()),
        (vec![Spell::Recharge], State::default()),
    ];
    let mut winners = Vec::new();
    for _ in 0..20 {
        winners.extend(try_permutations(&mut spells));
        // println!("Spells: {:?}", spells);
    }
    winners.iter().min_by_key(|&s| s).copied()
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut spells = vec![
        (vec![Spell::MagicMissile], State::default()),
        (vec![Spell::Drain], State::default()),
        (vec![Spell::Shield], State::default()),
        (vec![Spell::Poison], State::default()),
        (vec![Spell::Recharge], State::default()),
    ];
    let mut winners = Vec::new();
    for _ in 0..100 {
        winners.extend(try_permutations_hard(&mut spells));
    }
    winners.iter().min_by_key(|&s| s.1).map(|w| w.1)
}
