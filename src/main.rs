use goblin_fightclub::*;

fn main() {
    let gob1 = birth_goblin(String::from("Gob 1"));
    let gob2 = birth_goblin(String::from("Gob 2"));
    battle(gob1, gob2)
}
