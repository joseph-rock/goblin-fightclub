use rand::{thread_rng, Rng};

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

    fn roll(self) -> Option<u8> {
        if self.amount == 0 || self.sides == 0 {
            return None;
        }
        let mut rng = thread_rng();
        let mut total = 0;

        for _ in 0..self.amount {
            total += rng.gen_range(1..=self.sides);
        }

        Some(total + self.modifier)
    }

    fn description(self) -> String {
        let desc = self.amount.to_string() + "d" + &self.sides.to_string();
        if self.modifier == 0 {
            return desc;
        }
        desc + " +" + &self.modifier.to_string()
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
