use rand::{thread_rng, Rng};

#[derive(Clone, Copy, Debug)]
struct Dice {
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

    fn roll(self) -> Option<RollResult> {
        if self.amount == 0 || self.sides == 0 {
            return None;
        }
        let mut rng = thread_rng();
        let mut rolls: Vec<u8> = Vec::new();

        for _ in 0..self.amount {
            rolls.push(rng.gen_range(1..=self.sides));
        }

        let total = rolls.iter().fold(0, |sum, x| sum + x);
        Some(RollResult { total, rolls })
    }

    fn description(self) -> String {
        let desc = self.amount.to_string() + "d" + &self.sides.to_string();
        if self.modifier == 0 {
            return desc;
        }
        desc + " +" + &self.modifier.to_string()
    }
}

struct RollResult {
    total: u8,
    rolls: Vec<u8>,
}

struct Weapon {
    name: String,
    attack_dice: Dice,
}

impl Weapon {
    fn new(name: String, attack_dice: Dice) -> Weapon {
        Weapon {name, attack_dice}
    }
}

enum CommonWeapon {
    Dagger,
    Shortsword,
    Warhammer,
    Greatsword,
    Halberd,
    Greataxe
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

fn main() {
    todo!();
}
