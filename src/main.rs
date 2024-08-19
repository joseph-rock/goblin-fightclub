use goblin_fightclub::*;

fn main() {
    let gob1 = birth_goblin(String::from("Gob 1"));
    let gob2 = birth_goblin(String::from("Gob 2"));
    let _ = battle(gob1, gob2);
    ()
}

fn battle(attacker: Goblin, defender: Goblin) -> Goblin {
    println!("----------------------\n");
    dbg!(&attacker, &defender);
    println!("{} attacks {}", attacker.name, defender.name);

    let attack_result = attack_round(&attacker, defender);
    let defender = match attack_result {
        AttackRollResult::Hit {
            defender,
            d20_roll,
            damage_roll,
        } => {
            println!(
                "{} rolls {} - Hit for {}",
                attacker.name, d20_roll, damage_roll
            );
            defender
        }
        AttackRollResult::Miss { defender, d20_roll } => {
            println!("{} rolls {} - Miss", attacker.name, d20_roll);
            defender
        }
    };

    if defender.current_health <= 0 {
        println!("{} died\n", defender.name);
        return attacker;
    }
    battle(defender, attacker)
}

fn attack_round(attacker: &Goblin, defender: Goblin) -> AttackRollResult {
    let d20_roll = Dice::roll_d20();

    if d20_roll < defender.defense {
        return AttackRollResult::Miss {
            defender: defender,
            d20_roll,
        };
    }

    let damage_roll = attacker.attack();

    AttackRollResult::Hit {
        defender: defender.take_damage(damage_roll),
        d20_roll,
        damage_roll,
    }
}

pub enum AttackRollResult {
    Hit {
        defender: Goblin,
        d20_roll: u8,
        damage_roll: u8,
    },
    Miss {
        defender: Goblin,
        d20_roll: u8,
    },
}