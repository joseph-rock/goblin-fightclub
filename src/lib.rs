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

    fn roll_d20() -> u8 {
        let d20 = Dice::simple(20);
        d20.roll().unwrap()
    }

    pub fn description(self) -> String {
        let desc = self.amount.to_string() + "d" + &self.sides.to_string();
        if self.modifier == 0 {
            return desc;
        }
        desc + " +" + &self.modifier.to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Weapon {
    pub name: String,
    pub attack_dice: Dice,
}

impl Weapon {
    fn new(name: String, attack_dice: Dice) -> Weapon {
        Weapon { name, attack_dice }
    }
}

enum CommonWeapon {
    Dagger,
    Shortsword,
    Warhammer,
    Greatsword,
    Halberd,
    Greataxe,
}

impl CommonWeapon {
    fn new(self) -> Weapon {
        match self {
            CommonWeapon::Dagger => Weapon::new(String::from("Dagger"), Dice::simple(4)),
            CommonWeapon::Shortsword => Weapon::new(String::from("Shortsword"), Dice::simple(6)),
            CommonWeapon::Warhammer => Weapon::new(String::from("Warhammer"), Dice::simple(8)),
            CommonWeapon::Greatsword => Weapon::new(String::from("Greatsword"), Dice::new(2, 6, 0)),
            CommonWeapon::Halberd => Weapon::new(String::from("Halberd"), Dice::simple(10)),
            CommonWeapon::Greataxe => Weapon::new(String::from("Greataxe"), Dice::simple(12)),
        }
    }
}

fn random_weapon() -> Weapon {
    let max = 6;
    let dice = Dice::simple(max);
    let result = dice.roll().unwrap();

    match result {
        1 => CommonWeapon::Dagger.new(),
        2 => CommonWeapon::Shortsword.new(),
        3 => CommonWeapon::Warhammer.new(),
        4 => CommonWeapon::Greatsword.new(),
        5 => CommonWeapon::Halberd.new(),
        6 => CommonWeapon::Greataxe.new(),
        _ => CommonWeapon::Dagger.new(),
    }
}

#[derive(Clone, Debug)]
pub struct Goblin {
    pub name: String,
    pub max_health: u8,
    pub current_health: i8,
    pub weapon: Weapon,
    pub defense: u8,
    pub wins: u8,
}

impl Goblin {
    pub fn attacks(&self, defender: &Goblin) -> AttackResult {
        let attack_roll = Dice::roll_d20();
        if attack_roll < defender.defense {
            return AttackResult::Miss { attack_roll };
        }

        let damage_roll = self.damage_roll();

        if attack_roll == 20 {
            let crit_attack_roll = Dice::roll_d20();
            if crit_attack_roll >= defender.defense {
                let crit_damage_roll = self.damage_roll();
                let total = damage_roll + crit_damage_roll;
                return AttackResult::Crit { damage_roll: total };
            }
        }

        AttackResult::Hit {
            attack_roll,
            damage_roll,
        }
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

pub fn birth_goblin(name: String) -> Goblin {
    let health_dice = Dice::new(2, 20, 5);
    let health = health_dice.roll().unwrap();
    let defense = Dice::simple(18).roll().unwrap();
    let weapon = random_weapon();

    Goblin {
        name,
        max_health: health,
        current_health: health as i8,
        weapon,
        defense,
        wins: 0,
    }
}

pub enum AttackResult {
    Miss { attack_roll: u8 },
    Hit { attack_roll: u8, damage_roll: u8 },
    Crit { damage_roll: u8 },
}
