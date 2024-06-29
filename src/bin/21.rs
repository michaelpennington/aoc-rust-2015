advent_of_code::solution!(21);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Warrior {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl Warrior {
    fn fight(&self, enemy: &Warrior) -> bool {
        let mut w1 = *self;
        let mut w2 = *enemy;
        loop {
            if w2.hp == 0 {
                break true;
            } else if w1.hp == 0 {
                break false;
            }
            w2.hp = w2
                .hp
                .saturating_sub(w1.damage.saturating_sub(w2.armor).max(1));
            w1.hp = w1
                .hp
                .saturating_sub(w2.damage.saturating_sub(w1.armor).max(1));
        }
    }

    fn from_kit_with_hp(kit: &Kit, hp: u32) -> Self {
        let (damage, armor) = kit.stats();
        Self { hp, damage, armor }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Kit {
    weapon: Weapon,
    armor: Armor,
    rings: Rings,
}

impl Kit {
    const fn stats(&self) -> (u32, u32) {
        let w = self.weapon.stats();
        let a = self.armor.stats();
        let r = self.rings.stats();
        (w.0 + a.0 + r.0, w.1 + a.1 + r.1)
    }

    const fn cost(&self) -> u32 {
        self.weapon.cost() + self.armor.cost() + self.rings.cost()
    }

    fn iter() -> impl Iterator<Item = Self> {
        WEAPONS.iter().flat_map(move |&weapon| {
            ARMORS.iter().flat_map(move |&armor| {
                POSSIBLE_RINGS.iter().map(move |&rings| Self {
                    weapon,
                    armor,
                    rings,
                })
            })
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Weapon {
    Dagger,
    Shortsword,
    Warhammer,
    Longsword,
    Greataxe,
}

impl Weapon {
    const fn stats(&self) -> (u32, u32) {
        match self {
            Weapon::Dagger => (4, 0),
            Weapon::Shortsword => (5, 0),
            Weapon::Warhammer => (6, 0),
            Weapon::Longsword => (7, 0),
            Weapon::Greataxe => (8, 0),
        }
    }

    const fn cost(&self) -> u32 {
        match self {
            Weapon::Dagger => 8,
            Weapon::Shortsword => 10,
            Weapon::Warhammer => 25,
            Weapon::Longsword => 40,
            Weapon::Greataxe => 74,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Armor {
    None,
    Leather,
    Chainmail,
    Splintmail,
    Bandedmail,
    Platemail,
}

impl Armor {
    const fn stats(&self) -> (u32, u32) {
        match self {
            Armor::None => (0, 0),
            Armor::Leather => (0, 1),
            Armor::Chainmail => (0, 2),
            Armor::Splintmail => (0, 3),
            Armor::Bandedmail => (0, 4),
            Armor::Platemail => (0, 5),
        }
    }

    const fn cost(&self) -> u32 {
        match self {
            Armor::None => 0,
            Armor::Leather => 13,
            Armor::Chainmail => 31,
            Armor::Splintmail => 53,
            Armor::Bandedmail => 75,
            Armor::Platemail => 102,
        }
    }
}

const WEAPONS: [Weapon; 5] = [
    Weapon::Dagger,
    Weapon::Shortsword,
    Weapon::Warhammer,
    Weapon::Longsword,
    Weapon::Greataxe,
];

const ARMORS: [Armor; 6] = [
    Armor::None,
    Armor::Leather,
    Armor::Chainmail,
    Armor::Splintmail,
    Armor::Bandedmail,
    Armor::Platemail,
];

const POSSIBLE_RINGS: [Rings; 22] = [
    Rings::None,
    Rings::One(Ring::Da1),
    Rings::One(Ring::Da2),
    Rings::One(Ring::Da3),
    Rings::One(Ring::De1),
    Rings::One(Ring::De2),
    Rings::One(Ring::De3),
    Rings::Two(Ring::Da1, Ring::Da2),
    Rings::Two(Ring::Da1, Ring::Da3),
    Rings::Two(Ring::Da1, Ring::De1),
    Rings::Two(Ring::Da1, Ring::De2),
    Rings::Two(Ring::Da1, Ring::De3),
    Rings::Two(Ring::Da2, Ring::Da3),
    Rings::Two(Ring::Da2, Ring::De1),
    Rings::Two(Ring::Da2, Ring::De2),
    Rings::Two(Ring::Da2, Ring::De3),
    Rings::Two(Ring::Da3, Ring::De1),
    Rings::Two(Ring::Da3, Ring::De2),
    Rings::Two(Ring::Da3, Ring::De3),
    Rings::Two(Ring::De1, Ring::De2),
    Rings::Two(Ring::De1, Ring::De3),
    Rings::Two(Ring::De2, Ring::De3),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rings {
    None,
    One(Ring),
    Two(Ring, Ring),
}

impl Rings {
    const fn stats(&self) -> (u32, u32) {
        match self {
            Rings::None => (0, 0),
            Rings::One(r1) => r1.stats(),
            Rings::Two(r1, r2) => (r1.stats().0 + r2.stats().0, r1.stats().1 + r2.stats().1),
        }
    }

    const fn cost(&self) -> u32 {
        match self {
            Rings::None => 0,
            Rings::One(r1) => r1.cost(),
            Rings::Two(r1, r2) => r1.cost() + r2.cost(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ring {
    Da1,
    Da2,
    Da3,
    De1,
    De2,
    De3,
}

impl Ring {
    const fn stats(&self) -> (u32, u32) {
        match self {
            Ring::Da1 => (1, 0),
            Ring::Da2 => (2, 0),
            Ring::Da3 => (3, 0),
            Ring::De1 => (0, 1),
            Ring::De2 => (0, 2),
            Ring::De3 => (0, 3),
        }
    }

    const fn cost(&self) -> u32 {
        match self {
            Ring::Da1 => 25,
            Ring::Da2 => 50,
            Ring::Da3 => 100,
            Ring::De1 => 20,
            Ring::De2 => 40,
            Ring::De3 => 80,
        }
    }
}

pub fn part_one(_input: &str) -> Option<u32> {
    let enemy = Warrior {
        hp: 103,
        damage: 9,
        armor: 2,
    };
    Kit::iter()
        .filter(|kit| Warrior::from_kit_with_hp(kit, 100).fight(&enemy))
        .map(|kit| kit.cost())
        .min()
}

pub fn part_two(_input: &str) -> Option<u32> {
    let enemy = Warrior {
        hp: 103,
        damage: 9,
        armor: 2,
    };
    Kit::iter()
        .filter(|kit| !Warrior::from_kit_with_hp(kit, 100).fight(&enemy))
        .map(|kit| kit.cost())
        .max()
}
