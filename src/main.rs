use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
struct Dice {
    amount: u8,
    sides: u8,
    modifier: u8,
}

impl Dice {
    fn simple(sides: u8) -> Dice {
        Dice {
            amount: 1,
            sides,
            modifier: 0,
        }
    }

    fn parse(dice_str: &str) -> Dice {
        todo!()
    }

    fn roll(self) -> Option<u8> {
        if self.amount == 0 || self.sides == 0 {
            return None;
        }
        let min = self.amount;
        let max = self.sides * self.amount;

        let mut rng = thread_rng();
        Some(rng.gen_range(min..=max) + self.modifier)
    }
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

fn main() {
    todo!();
}
