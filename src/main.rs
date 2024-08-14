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

enum CommonDice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

impl CommonDice {
    fn new(self) -> Dice {
        match self {
            CommonDice::D4 => Dice::simple(4),
            CommonDice::D6 => Dice::simple(6),
            CommonDice::D8 => Dice::simple(8),
            CommonDice::D10 => Dice::simple(10),
            CommonDice::D12 => Dice::simple(12),
            CommonDice::D20 => Dice::simple(20),
        }
    }
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
            CommonWeapon::Dagger => Weapon::new(String::from("Dagger"), CommonDice::D4.new()),
            CommonWeapon::Shortsword => Weapon::new(String::from("Shortsword"), CommonDice::D6.new()),
            CommonWeapon::Warhammer => Weapon::new(String::from("Warhammer"), CommonDice::D8.new()),
            CommonWeapon::Greatsword => Weapon::new(String::from("Greatsword"), Dice::new(2, 6, 0)),
            CommonWeapon::Halberd => Weapon::new(String::from("Halberd"), CommonDice::D10.new()),
            CommonWeapon::Greataxe => Weapon::new(String::from("Greataxe"), CommonDice::D12.new()),
        }
    }
}

fn main() {
    todo!();
}
