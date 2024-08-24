use goblin_fightclub::*;
use std::{thread, time};

fn main() {
    let mut champion = birth_goblin(String::from("Gob 0"));
    let mut i = 1;

    loop {
        champion = battle(champion, i);
        i += 1;
        pause();
    }
}

fn battle(mut champion: Goblin, i: u8) -> Goblin {
    let mut challenger = birth_goblin(format!("Gob {i}"));
    let mut log: Vec<String> = Vec::new();
    update_display(&champion, &challenger, &log);

    loop {
        let is_killing_blow = attack_log(&mut champion, &mut challenger, &mut log);
        update_display(&champion, &challenger, &log);
        if is_killing_blow {
            return champion;
        }

        let is_killing_blow = attack_log(&mut challenger, &mut champion, &mut log);
        update_display(&champion, &challenger, &log);
        if is_killing_blow {
            return challenger;
        }
    }
}

fn attack_log(attacker: &mut Goblin, defender: &mut Goblin, log: &mut Vec<String>) -> bool {
    let result = attacker.attacks(&defender);
    let _ = match result {
        AttackResult::Hit {
            attack_roll: ar,
            damage_roll: dr,
        } => {
            defender.take_damage(dr);
            log.push(format!("{} rolls {ar} - Hit for {dr}", attacker.name));
        }
        AttackResult::Crit { damage_roll: dr } => {
            defender.take_damage(dr);
            log.push(format!("{} Critical Hit! for {dr}", attacker.name));
        }
        AttackResult::Miss { attack_roll: ar } => {
            log.push(format!("{} rolls {ar} - Miss", attacker.name));
        }
    };

    if defender.current_health <= 0 {
        log.push(format!("{} Died", defender.name));
        attacker.win();
        return true;
    }

    false
}

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn pause() -> () {
    let pause_len = time::Duration::from_millis(1000);
    thread::sleep(pause_len);
}

fn update_display(champion: &Goblin, challenger: &Goblin, log: &Vec<String>) -> () {
    clear();
    print_header(&champion, &challenger);
    for line in log {
        println!("{line}");
    }
    pause();
}

fn print_header(left: &Goblin, right: &Goblin) -> () {
    let lweapon = left.weapon.attack_dice.description();
    let rweapon = right.weapon.attack_dice.description();

    println!("{:<15} |   {:<15}", left.name, right.name);
    println!("{:<15} |   {:<15}", fmt_wins(left), fmt_wins(right));
    println!("{:<15} |   {:<15}", fmt_hp(left), fmt_hp(right));
    println!("{:<15} |   {:<15}", fmt_def(left), fmt_def(right));
    println!("{:<15} |   {:<15}", left.weapon.name, right.weapon.name);
    println!("{:<15} |   {:<15}", lweapon, rweapon);
    println!();
}

fn fmt_wins(gob: &Goblin) -> String {
    "Wins: ".to_owned() + &gob.wins.to_string()
}

fn fmt_hp(gob: &Goblin) -> String {
    "HP: ".to_owned() + &gob.current_health.to_string() + "/" + &gob.max_health.to_string()
}

fn fmt_def(gob: &Goblin) -> String {
    "Def: ".to_owned() + &gob.defense.to_string()
}
