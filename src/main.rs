use goblin_fightclub::*;

fn main() {
    let gob1 = birth_goblin(String::from("Gob 1"));
    let gob2 = birth_goblin(String::from("Gob 2"));
    let _ = battle(gob1, gob2);
    ()
}
