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
        let champion_attack = Dice::roll_d20();
        if champion_attack >= challenger.defense {
            let champion_damage = champion.damage_roll();
            challenger.take_damage(champion_damage);
            log.push(format!(
                "{} attacks - Roll {} - Hit for {}",
                champion.name, champion_attack, champion_damage
            ));
            update_display(&champion, &challenger, &log);
            if challenger.current_health <= 0 {
                log.push(format!("{} Died", challenger.name));
                update_display(&champion, &challenger, &log);
                champion.win();
                return champion;
            }
        } else {
            log.push(format!(
                "{} attacks - Roll {} - Miss",
                champion.name, champion_attack
            ));
            update_display(&champion, &challenger, &log);
        }

        let challenger_attack = Dice::roll_d20();
        if challenger_attack >= champion.defense {
            let challenger_damage = challenger.damage_roll();
            champion.take_damage(challenger_damage);
            log.push(format!(
                "{} attacks - Roll {} - Hit for {}",
                challenger.name, challenger_attack, challenger_damage
            ));
            update_display(&champion, &challenger, &log);
            if champion.current_health <= 0 {
                log.push(format!("{} Died", champion.name));
                update_display(&champion, &challenger, &log);
                challenger.win();
                return challenger;
            }
        } else {
            log.push(format!(
                "{} attacks - Roll {} - Miss",
                challenger.name, challenger_attack
            ));
            update_display(&champion, &challenger, &log);
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
    println!("{:<15} |   {:<15}", left.name, right.name);
    println!("{:<15} |   {:<15}", fmt_wins(left), fmt_wins(right));
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

fn fmt_wins(gob: &Goblin) -> String {
    "Wins: ".to_owned() + &gob.wins.to_string()
}

fn fmt_hp(gob: &Goblin) -> String {
    "HP: ".to_owned() + &gob.current_health.to_string() + "/" + &gob.max_health.to_string()
}

fn fmt_def(gob: &Goblin) -> String {
    "Def: ".to_owned() + &gob.defense.to_string()
}
