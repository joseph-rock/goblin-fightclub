use rand::{thread_rng, Rng};

struct RollResult {
    total: u8,
    rolls: Vec<u8>,
}

enum AttackResult {
    Hit {
        defender: Goblin,
        roll: u8,
        roll_result: RollResult,
    },
    Miss {
        defender: Goblin,
        roll: u8,
    },
}

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

        let total = rolls.iter().fold(0, |sum, x| sum + x) + self.modifier;
        Some(RollResult { total, rolls })
    }

    // TODO: Add crit system
    fn roll_d20(_diffuculty_score: &u8) -> u8 {
        let d20 = Dice::simple(20);
        d20.roll().unwrap().total
    }

    fn description(self) -> String {
        let desc = self.amount.to_string() + "d" + &self.sides.to_string();
        if self.modifier == 0 {
            return desc;
        }
        desc + " +" + &self.modifier.to_string()
    }
}

#[derive(Clone, Debug)]
struct Weapon {
    name: String,
    attack_dice: Dice,
}

impl Weapon {
    fn new(name: String, attack_dice: Dice) -> Weapon {
        Weapon { name, attack_dice }
    }

    fn random_weapon() -> Weapon {
        let max = 6;
        let dice = Dice::simple(max);
        let result = dice.roll().unwrap().total;

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

#[derive(Clone, Debug)]
pub struct Goblin {
    name: String,
    max_health: u8,
    current_health: i8,
    weapon: Weapon,
    defense: u8,
    wins: u8,
}

impl Goblin {
    fn attack(&self) -> RollResult {
        let roll = self.weapon.attack_dice.roll();
        match roll {
            Some(result) => result,
            None => RollResult {
                total: 1,
                rolls: [1].to_vec(),
            },
        }
    }

    fn take_damage(self, damage: u8) -> Goblin {
        Goblin {
            name: self.name,
            max_health: self.max_health,
            current_health: self.current_health - (damage as i8),
            weapon: self.weapon,
            defense: self.defense,
            wins: self.wins,
        }
    }

    fn defend(self, attacker: &Goblin) -> AttackResult {
        let roll_to_hit = Dice::roll_d20(&self.defense);

        if roll_to_hit < self.defense {
            return AttackResult::Miss {
                defender: self,
                roll: roll_to_hit,
            };
        }

        let damage_roll = attacker.attack();

        AttackResult::Hit {
            defender: self.take_damage(damage_roll.total),
            roll: roll_to_hit,
            roll_result: damage_roll,
        }
    }
}

pub fn birth_goblin(name: String) -> Goblin {
    let health_dice = Dice::new(2, 20, 5);
    let health = health_dice.roll().unwrap().total;
    let defense = Dice::simple(18).roll().unwrap().total;
    let weapon = Weapon::random_weapon();

    Goblin {
        name,
        max_health: health,
        current_health: health as i8,
        weapon,
        defense,
        wins: 0,
    }
}

pub fn battle(attacker: Goblin, defender: Goblin) {
    println!("----------------------\n");
    dbg!(&attacker, &defender);
    println!("{} attacks {}", attacker.name, defender.name);

    let attack_result = defender.defend(&attacker);
    match attack_result {
        AttackResult::Hit {
            defender,
            roll,
            roll_result,
        } => {
            println!(
                "{} rolls {} - Hit for {}",
                attacker.name, roll, roll_result.total
            );
            if defender.current_health <= 0 {
                println!("{} died\n", defender.name);
                return ();
            }
            return battle(defender, attacker);
        }
        AttackResult::Miss { defender, roll } => {
            println!("{} rolls {} - Miss", attacker.name, roll);
            return battle(defender, attacker);
        }
    }
}
