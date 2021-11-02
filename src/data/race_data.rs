use serde_derive::{Deserialize, Serialize};

use super::json::Choice;

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct RaceData {
    pub name: String,
    pub speed: u8,
    pub traits: Vec<TraitOption>
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum TraitOption {
    Stat(StatTrait),
    StatOption(StatOptionTrait),
    Trait(SingleTrait)
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct StatOptionTrait {
    pub choice: Choice,
    pub value: u8
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct SingleTrait {
    pub name: String
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct StatTrait {
    pub stat: String,
    pub value: u8
}

