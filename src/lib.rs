use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Debug)]
pub struct Dice {
    amount: u8,
    sides: u8,
    modifier: u8,
}

impl Dice {
    fn new(amount: u8, sides: u8, modifier: u8) -> Dice {
        Dice {
            amount,
            sides,
            modifier,
        }
    }

    fn simple(sides: u8) -> Dice {
        Dice {
            amount: 1,
            sides,
            modifier: 0,
        }
    }

    fn roll(self) -> Option<u8> {
        if self.amount == 0 || self.sides == 0 {
            return None;
        }
        let mut rng = thread_rng();
        let mut rolls: Vec<u8> = Vec::new();

        for _ in 0..self.amount {
            rolls.push(rng.gen_range(1..=self.sides));
        }

        let total = rolls.iter().fold(0, |sum, x| sum + x) + self.modifier;
        Some(total)
    }

    fn roll_d20(ability_modifier: &u8, difficulty_score: &u8) -> D20Roll {
        let d20_roll = Dice::simple(20).roll().unwrap();
        let total_roll = d20_roll + ability_modifier;

        match d20_roll {
            1 => D20Roll::CriticalFailure(d20_roll),
            20 => D20Roll::CriticalSuccess(d20_roll),
            _ if total_roll >= *difficulty_score => D20Roll::Hit(total_roll),
            _ => D20Roll::Miss(total_roll),
        }
    }

    pub fn description(self) -> String {
        let desc = self.amount.to_string() + "d" + &self.sides.to_string();
        if self.modifier != 0 {
            return desc + "+" + &self.modifier.to_string();
        }
        desc
    }
}

#[derive(Clone, Debug)]
pub struct Goblin {
    pub name: String,
    pub level: u8,
    pub max_health: u8,
    pub current_health: i8,
    pub weapon: Weapon,
    pub defense: u8,
    pub wins: u8,
    pub heals_available: u8,
}

impl Goblin {
    pub fn attacks(&self, defender: &Goblin) -> AttackResult {
        let attack_roll = Dice::roll_d20(&self.level, &defender.defense);
        let damage_roll = self.damage_roll();

        match attack_roll {
            D20Roll::Miss(roll) | D20Roll::CriticalFailure(roll) => {
                AttackResult::Miss { attack_roll: roll }
            }
            D20Roll::Hit(roll) => AttackResult::Hit {
                attack_roll: roll,
                damage_roll,
            },
            D20Roll::CriticalSuccess(roll) => {
                let crit_attack_roll = Dice::roll_d20(&self.level, &defender.defense);
                match crit_attack_roll {
                    D20Roll::Miss(_) | D20Roll::CriticalFailure(_) => AttackResult::Hit {
                        attack_roll: roll,
                        damage_roll,
                    },
                    D20Roll::Hit(_) | D20Roll::CriticalSuccess(_) => {
                        let crit_damage_roll = self.damage_roll();
                        let total = damage_roll + crit_damage_roll;
                        AttackResult::Crit { damage_roll: total }
                    }
                }
            }
        }
    }

    pub fn heal(&mut self) -> Option<u8> {
        if self.heals_available == 0 {
            return None;
        }

        let d6_roll = Dice::simple(6).roll().unwrap();
        self.heals_available -= 1;
        self.current_health += d6_roll as i8;
        Some(d6_roll)
    }

    fn damage_roll(&self) -> u8 {
        let roll = self.weapon.attack_dice.roll();
        match roll {
            Some(result) => result,
            None => 1,
        }
    }

    pub fn take_damage(&mut self, damage: u8) -> () {
        self.current_health -= damage as i8;
    }

    pub fn win(&mut self) -> () {
        self.wins += 1;
    }
}

#[derive(Clone, Debug)]
pub struct Weapon {
    name: String,
    modifier: u8,
    pub attack_dice: Dice,
}

impl Weapon {
    fn new(name: String, modifier: u8, attack_dice: Dice) -> Weapon {
        Weapon {
            name,
            modifier,
            attack_dice,
        }
    }

    pub fn name(&self) -> String {
        if self.modifier != 0 {
            return format!("{} +{}", self.name, self.modifier);
        }
        self.name.clone()
    }
}

pub enum AttackResult {
    Miss { attack_roll: u8 },
    Hit { attack_roll: u8, damage_roll: u8 },
    Crit { damage_roll: u8 },
}

enum D20Roll {
    Hit(u8),
    Miss(u8),
    CriticalSuccess(u8),
    CriticalFailure(u8),
}

enum CommonWeapon {
    Dagger,
    Shortsword,
    Warhammer,
    Greatsword,
    Halberd,
    Greataxe,
}

#[rustfmt::skip]
impl CommonWeapon {
    fn new(self, modifier: u8) -> Weapon {
        match self {
            CommonWeapon::Dagger => Weapon::new(
                String::from("Dagger"), 
                modifier, 
                Dice::new(1, 4, modifier)
            ), 
            CommonWeapon::Shortsword => Weapon::new(
                String::from("Shortsword"),
                modifier,
                Dice::new(1, 6, modifier),
            ),
            CommonWeapon::Warhammer => Weapon::new(
                String::from("Warhammer"),
                modifier,
                Dice::new(1, 8, modifier),
            ),
            CommonWeapon::Greatsword => Weapon::new(
                String::from("Greatsword"),
                modifier,
                Dice::new(2, 6, modifier),
            ),
            CommonWeapon::Halberd => Weapon::new(
                String::from("Halberd"),
                modifier,
                Dice::new(1, 10, modifier),
            ),
            CommonWeapon::Greataxe => Weapon::new(
                String::from("Greataxe"),
                modifier,
                Dice::new(1, 12, modifier),
            ),
        }
    }
}

fn random_weapon() -> Weapon {
    let max = 6;
    let dice = Dice::simple(max);
    let result = dice.roll().unwrap();

    let d20 = Dice::roll_d20(&0, &18);
    let modifier = match d20 {
        D20Roll::Hit(_) => Dice::simple(4).roll().unwrap(),
        D20Roll::CriticalSuccess(_) => 5,
        _ => 0,
    };

    match result {
        1 => CommonWeapon::Dagger.new(modifier),
        2 => CommonWeapon::Shortsword.new(modifier),
        3 => CommonWeapon::Warhammer.new(modifier),
        4 => CommonWeapon::Greatsword.new(modifier),
        5 => CommonWeapon::Halberd.new(modifier),
        6 => CommonWeapon::Greataxe.new(modifier),
        _ => CommonWeapon::Dagger.new(modifier),
    }
}

pub fn birth_goblin(name: String) -> Goblin {
    // TODO: come up with a better progression system, I like using levels
    let level = Dice::simple(4).roll().unwrap();

    let health = Dice::new(level, 20, level).roll().unwrap();
    let heals_available = Dice::simple(4).roll().unwrap();

    // defense range: 4-16 + level
    let defense = Dice::new(4, 4, level).roll().unwrap();
    let weapon = random_weapon();

    Goblin {
        name,
        level,
        max_health: health,
        current_health: health as i8,
        weapon,
        defense,
        wins: 0,
        heals_available,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dice_display() {
        let new_dice = Dice::new(4, 4, 4).description();
        let simple_dice = Dice::simple(4).description();
        assert_eq!(new_dice, "4d4+4");
        assert_eq!(simple_dice, "1d4");
    }

    #[test]
    fn weapon_display() {
        let base_weapon = CommonWeapon::Dagger.new(0);
        let mod_weapon = CommonWeapon::Dagger.new(4);
        assert_eq!(base_weapon.name(), "Dagger");
        assert_eq!(mod_weapon.name(), "Dagger +4");
    }
}
