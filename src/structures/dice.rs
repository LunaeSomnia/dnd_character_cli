#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum Dice {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100
}

impl Dice {
    pub fn from_str(s: &str) -> Option<Dice> {
        match s {
            "1d4" | "d4" | "D4" => Some(Dice::D4),
            "1d6" | "d6" | "D6" => Some(Dice::D6),
            "1d8" | "d8" | "D8"=> Some(Dice::D8),
            "1d10" | "d10" | "D10"=> Some(Dice::D10),
            "1d12" | "d12" | "D12"=> Some(Dice::D12),
            "1d20" | "d20" | "D20"=> Some(Dice::D20),
            "1d100" | "d100" | "D100" => Some(Dice::D100),
            _ => None
        }
    }

    pub fn to_num(&self) -> u8 {
        match &self {
            Dice::D4 => 4,
            Dice::D6 => 6,
            Dice::D8 => 8,
            Dice::D10 => 10,
            Dice::D12 => 12,
            Dice::D20 => 20,
            Dice::D100 => 100
        }
    }

    pub fn less_than(&self, dice: &Dice) -> bool {
        self < dice
    }

    pub fn less_equal_than(&self, dice: &Dice) -> bool {
        self <= dice
    }
}