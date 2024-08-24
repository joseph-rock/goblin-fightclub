use goblin_fightclub::*;
use std::{thread, time};

fn main() {
    let mut champion = birth_goblin(String::from("Gob 0"));
    let mut i = 1;

    loop {
        champion = battle(champion, i);
        i += 1;
    }
}

fn battle(mut champion: Goblin, i: u8) -> Goblin {
    let mut challenger = birth_goblin(format!("Gob {i}"));
    let mut log: Vec<String> = Vec::new();
    update_display(&champion, &challenger, &log);

    loop {
        let champion_attack = champion.attack(&challenger);
        let _ = match champion_attack {
            AttackResult::Hit {
                attack_roll: ar,
                damage_roll: dr,
            } => {
                challenger.take_damage(dr);
                log.push(format!("{} rolls {ar} - Hit for {dr}", champion.name));
            }
            AttackResult::Crit { damage_roll: dr } => {
                challenger.take_damage(dr);
                log.push(format!("{} Critical Hit! for {dr}", champion.name));
            }
            AttackResult::Miss { attack_roll: ar } => {
                log.push(format!("{} rolls {ar} - Miss", champion.name));
            }
        };
        update_display(&champion, &challenger, &log);

        if challenger.current_health <= 0 {
            log.push(format!("{} Died", challenger.name));
            update_display(&champion, &challenger, &log);
            champion.win();
            return champion;
        }

        let challenger_attack = challenger.attack(&champion);
        let _ = match challenger_attack {
            AttackResult::Hit {
                attack_roll: ar,
                damage_roll: dr,
            } => {
                champion.take_damage(dr);
                log.push(format!("{} rolls {ar} - Hit for {dr}", challenger.name));
            }
            AttackResult::Crit { damage_roll: dr } => {
                champion.take_damage(dr);
                log.push(format!("{} Critical Hit! for {dr}", challenger.name));
            }
            AttackResult::Miss { attack_roll: ar } => {
                log.push(format!("{} rolls {ar} - Miss", challenger.name));
            }
        };
        update_display(&champion, &challenger, &log);

        if champion.current_health <= 0 {
            log.push(format!("{} Died", champion.name));
            update_display(&champion, &challenger, &log);
            challenger.win();
            return challenger;
        }
    }
}

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn update_display(champion: &Goblin, challenger: &Goblin, log: &Vec<String>) -> () {
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
