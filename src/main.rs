use goblin_fightclub::*;
use std::{thread, time};

fn main() {
    let seed = birth_goblin(String::from("Gob 0"));
    game_loop(seed, 1);
    ()
}

fn game_loop(champion: Goblin, i: u8) -> () {
    let challenger = birth_goblin(format!("Gob {i}"));
    let champion = battle(champion, challenger);
    game_loop(champion, i + 1);
}

fn battle(attacker: Goblin, defender: Goblin) -> Goblin {
    let mut log: Vec<String> = Vec::new();

    let attack_dialog = format!("{} attacks {}", attacker.name, defender.name);
    log.push(attack_dialog);
    display(&attacker, &defender, &log);

    let attack_result = attack_round(&attacker, defender);
    let defender = match attack_result {
        AttackRollResult::Hit {
            defender,
            d20_roll,
            damage_roll,
        } => {
            let hit_dialog = format!(
                "{} rolls {} - Hit for {}",
                attacker.name, d20_roll, damage_roll
            );
            log.push(hit_dialog);
            display(&attacker, &defender, &log);
            defender
        }
        AttackRollResult::Miss { defender, d20_roll } => {
            let miss_dialog = format!("{} rolls {} - Miss", attacker.name, d20_roll);
            log.push(miss_dialog);
            display(&attacker, &defender, &log);
            defender
        }
    };

    if defender.current_health <= 0 {
        let death = format!("{} died\n", defender.name);
        log.push(death);
        display(&attacker, &defender, &log);
        return attacker.win();
    }
    battle(defender, attacker)
}

fn attack_round(attacker: &Goblin, defender: Goblin) -> AttackRollResult {
    let d20_roll = Dice::roll_d20();

    if d20_roll < defender.defense {
        return AttackRollResult::Miss { defender, d20_roll };
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

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn display(champion: &Goblin, challenger: &Goblin, log: &Vec<String>) -> () {
    let pause_len = time::Duration::from_millis(1000);
    let pause = || thread::sleep(pause_len);
    clear();
    print_header(&champion, &challenger);
    for line in log {
        println!("{line}");
    }
    pause();
}

fn print_header(left: &Goblin, right: &Goblin) -> () {
    let mut left = left;
    let mut right = right;
    let mut foo = left;

    if left.wins == right.wins {
        if left.name < right.name {
            foo = right;
            right = left;
            left = foo;
        }
    }

    if right.wins > left.wins {
        foo = left;
        left = right;
        right = foo;
    }

    println!("{:<15} |   {:<15}", left.name, right.name);
    println!("{:<15} |   {:<15}", left.wins, right.wins);
    println!("{:<15} |   {:<15}", fmt_hp(left), fmt_hp(right));
    println!("{:<15} |   {:<15}", fmt_def(left), fmt_def(right));
    println!("{:<15} |   {:<15}", left.weapon.name, right.weapon.name);
    println!(
        "{:<15} |   {:<15}",
        left.weapon.attack_dice.description(),
        right.weapon.attack_dice.description()
    );
    println!();
}

fn fmt_hp(gob: &Goblin) -> String {
    "HP: ".to_owned() + &gob.current_health.to_string() + "/" + &gob.max_health.to_string()
}

fn fmt_def(gob: &Goblin) -> String {
    "Def: ".to_owned() + &gob.defense.to_string()
}
