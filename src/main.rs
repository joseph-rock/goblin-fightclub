use goblin_fightclub::{birth_goblin, AttackResult, Goblin};
use std::{thread, time};

fn main() {
    let mut champion = birth_goblin(String::from("Gob 0"));
    let mut generation = 1;

    loop {
        champion = battle(champion, generation);
        generation += 1;
        pause();
    }
}

fn battle(mut champion: Goblin, generation: u8) -> Goblin {
    let mut challenger = birth_goblin(format!("Gob {generation}"));
    let mut log = Vec::<String>::new();

    update_display(&champion, &challenger, &log);

    loop {
        let champion_won: bool = action_log(&mut champion, &mut challenger, &mut log);
        update_display(&champion, &challenger, &log);
        if champion_won {
            return champion;
        }

        let challenger_won: bool = action_log(&mut challenger, &mut champion, &mut log);
        update_display(&champion, &challenger, &log);
        if challenger_won {
            return challenger;
        }
    }
}

#[rustfmt::skip]
fn action_log(attacker: &mut Goblin, defender: &mut Goblin, log: &mut Vec<String>) -> bool {
    if attacker.current_health <= 10 && attacker.heals_available > 0 {
        let heal_amount = attacker.heal().unwrap().to_string();
        log.push(format!("{} heals for {heal_amount}", attacker.name));
    } else {
        let attack_result = attacker.attacks(&defender);
        let _ = match attack_result {
            AttackResult::Hit {
                attack_roll: ar,
                damage_roll: dr,
            } => {
                defender.take_damage(dr);
                log.push(format!("{} rolls {ar} - Hit for {dr}", attacker.name));
            }
            AttackResult::Crit { 
                damage_roll: dr 
            } => {
                defender.take_damage(dr);
                log.push(format!("{} Critical Hit! for {dr}", attacker.name));
            }
            AttackResult::Miss { 
                attack_roll: ar 
            } => {
                log.push(format!("{} rolls {ar} - Miss", attacker.name));
            }
        };
    }

    if defender.current_health <= 0 {
        log.push(format!("{} Died", defender.name));
        attacker.win();
        return true;
    }

    false
}

fn update_display(champion: &Goblin, challenger: &Goblin, log: &Vec<String>) -> () {
    clear();
    print_header(&champion, &challenger);
    for line in log {
        println!("{line}");
    }
    pause();
}

#[rustfmt::skip]
fn print_header(left: &Goblin, right: &Goblin) -> () {
    let fmt_level = |gob: &Goblin| "Level: ".to_owned() + &gob.level.to_string();
    let fmt_wins = |gob: &Goblin| "Wins: ".to_owned() + &gob.wins.to_string();
    let fmt_hp = |gob: &Goblin| "HP: ".to_owned() + &gob.current_health.to_string() + "/" + &gob.max_health.to_string();
    let fmt_def = |gob: &Goblin| "Def: ".to_owned() + &gob.defense.to_string();
    let fmt_heals = |gob: &Goblin| "Heals: ".to_owned() + &gob.heals_available.to_string();
    let lweapon = left.weapon.attack_dice.description();
    let rweapon = right.weapon.attack_dice.description();

    println!("{:<15} |   {:<15}", left.name, right.name);
    println!("{:<15} |   {:<15}", fmt_level(left), fmt_level(right));
    println!("{:<15} |   {:<15}", fmt_wins(left), fmt_wins(right));
    println!("{:<15} |   {:<15}", fmt_hp(left), fmt_hp(right));
    println!("{:<15} |   {:<15}", fmt_def(left), fmt_def(right));
    println!("{:<15} |   {:<15}", fmt_heals(left), fmt_heals(right));
    println!("{:<15} |   {:<15}", left.weapon.name(), right.weapon.name());
    println!("{:<15} |   {:<15}", lweapon, rweapon);
    println!();
}

fn clear() -> () {
    print!("\x1B[2J\x1B[1;1H");
}

fn pause() -> () {
    let pause_len = time::Duration::from_millis(1000);
    thread::sleep(pause_len);
}
