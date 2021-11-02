pub enum Alignment {
    LawfulGood = 0,
    LawfulNeutral = 1,
    LawfulEvil = 2,
    ChaoticGood = 3,
    ChaoticNeutral = 4,
    ChaoticEvil = 5,
    NeutralGood = 6,
    Neutral = 7,
    NeutralEvil = 8
}

impl Alignment {
    pub fn new(i: usize) -> Alignment {
        match i {
            0 => Alignment::LawfulGood,
            1 => Alignment::LawfulNeutral,
            2 => Alignment::LawfulEvil,
            3 => Alignment::ChaoticGood,
            4 => Alignment::ChaoticNeutral,
            5 => Alignment::ChaoticEvil,
            6 => Alignment::NeutralGood,
            8 => Alignment::NeutralEvil,
            _ => Alignment::Neutral
        }
    }
}

impl ToString for Alignment {
    fn to_string(&self) -> String {
        match self {
            Alignment::LawfulGood => String::from("Lawful Good"),
            Alignment::LawfulNeutral => String::from("Lawful Neutral"),
            Alignment::LawfulEvil => String::from("Lawful Evil"),
            Alignment::ChaoticGood => String::from("Chaotic Good"),
            Alignment::ChaoticNeutral => String::from("Chaotic Neutral"),
            Alignment::ChaoticEvil => String::from("Chaotic Evil"),
            Alignment::NeutralGood => String::from("Neutral Good"),
            Alignment::NeutralEvil => String::from("Neutral Evil"),
            Alignment::Neutral => String::from("True Neutral")
        }
    }
}