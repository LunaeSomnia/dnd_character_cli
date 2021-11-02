use std::vec;

use serde_derive::{Serialize, Deserialize};
use crate::data::race_data::StatTrait;

use super::colors::{Color, ToColor};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum StatType {
    Strength = 0,
    Dexterity = 1,
    Constitution = 2,
    Intelligence = 3,
    Wisdom = 4,
    Charisma = 5
}

impl StatType {
    pub fn new(i: u8) -> StatType {
        match i {
            0 => StatType::Strength,
            1 => StatType::Dexterity,
            2 => StatType::Constitution,
            3 => StatType::Intelligence,
            4 => StatType::Wisdom,
            _ => StatType::Charisma,
        }
    }

    pub fn to_abr(&self) -> String {
        match &self {
            StatType::Strength => String::from("STR"),
            StatType::Dexterity => String::from("DEX"),
            StatType::Constitution => String::from("CON"),
            StatType::Intelligence => String::from("INT"),
            StatType::Wisdom => String::from("WIS"),
            StatType::Charisma => String::from("CHA"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub stats: Vec<Stat>
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            stats: vec![
                Stat { stat_type: StatType::Strength, value: 0 },
                Stat { stat_type: StatType::Dexterity, value: 0 },
                Stat { stat_type: StatType::Constitution, value: 0 },
                Stat { stat_type: StatType::Intelligence, value: 0 },
                Stat { stat_type: StatType::Wisdom, value: 0 },
                Stat { stat_type: StatType::Charisma, value: 0 },
            ]
        }
    }

    pub fn get(&self, stat: StatType) -> Option<&Stat> {
        for s in &self.stats {
            if s.stat_type == stat {
                return Some(&s);
            }
        }
        return None
    }

    pub fn get_i(&self, stat: StatType) -> usize {
        for i in 0..self.stats.len() {
            if self.stats[i].stat_type == stat {
                return i;
            }
        }
        0
    }

    pub fn sum(&mut self, stat: StatType, x: u8) {
        let i = self.get_i(stat);
        self.stats[i].value += x;
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stat {
    pub stat_type: StatType,
    pub value: u8,
}

impl Stat {
    pub fn to_mod(&self) -> i8 {
        to_mod(self.value)
    }

    pub fn val_to_str(&self) -> String {
        if self.value < 10 {
            format!(" {}", self.value)
        } else {
            self.value.to_string()
        }
    }

    pub fn mod_to_str(&self) -> String {
        let mut modifier = self.to_mod().to_string();
        if !modifier.contains("-") {
            modifier = format!("+{}",self.to_mod());
        }
        modifier
    }
}

impl ToString for StatType {
    fn to_string(&self) -> String {
        match &self {
            StatType::Strength => String::from("Strength"),
            StatType::Dexterity => String::from("Dexterity"),
            StatType::Constitution => String::from("Constitution"),
            StatType::Intelligence => String::from("Intelligence"),
            StatType::Wisdom => String::from("Wisdom"),
            StatType::Charisma => String::from("Charisma"),
        }
    }
}

impl ToColor for StatType {
    fn to_color(&self, background: bool) -> Color {
        let color: Color = match &self {
            StatType::Strength => {
                if !background {
                    return Color::Red;
                } Color::RedBG
            },
            StatType::Dexterity => {
                if !background {
                    return Color::Yellow;
                } Color::YellowBG
            },
            StatType::Constitution => {
                if !background {
                    return Color::Blue;
                } Color::BlueBG
            },
            StatType::Intelligence => {
                if !background {
                    return Color::Cyan;
                } Color::CyanBG
            },
            StatType::Wisdom => {
                if !background {
                    return Color::Green;
                } Color::GreenBG
            },
            StatType::Charisma => {
                if !background {
                    return Color::Magenta;
                } Color::MagentaBG
            }
        };
        color
    }
}

impl StatType {
    pub fn from_str(str: &str) -> Option<StatType> {
        match str {
            "Strength" | "STR" => Some(StatType::Strength),
            "Dexterity" | "DEX" => Some(StatType::Dexterity),
            "Constitution" | "CON" => Some(StatType::Constitution),
            "Intelligence" | "INT "=> Some(StatType::Intelligence),
            "Wisdom" | "WIS " => Some(StatType::Wisdom),
            "Charisma" | "CHA "=> Some(StatType::Charisma),
            _ => None
        }
    }
}

pub fn to_mod(num: u8) -> i8 {
    (num as i8) / 2 -5
}

pub fn mod_to_str(num: i8) -> String {
    let mut modifier = num.to_string();
    if !modifier.contains("-") {
        modifier = format!("+{}", num);
    }
    modifier
}