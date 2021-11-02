use crate::{data::class_data::ClassData, rolls::random};

use super::dice::Dice;

#[derive(Clone)]
pub struct Class {
    pub class: String,
    pub hit_dice: Dice,
    pub sub_class: String,
    pub level: u8
}

impl Class {
    pub fn from_data(data: &ClassData, level: u8) -> Class {
        Class { 
            class: data.name.clone(),
            hit_dice: Dice::from_str(&data.hit_dice)
                .expect(&format!("Hit dice in class data is invalid: Found '{}'", data.hit_dice)),
            sub_class: data.subclasses[random(0, data.subclasses.len())].clone(),
            level: level
        }
    }
}