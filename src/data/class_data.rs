pub mod multiclass;

use multiclass::Multiclass;
use super::json::Choice;

use serde_derive::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct ClassData {
    pub name: String,
    pub hit_dice: String,
    pub proficiencies: Vec<ClassProficiency>,
    pub equipment: Vec<Choice>,
    pub subclasses: Vec<String>,
    pub multiclassing: Multiclass,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct ClassProficiency {
    pub prof_type: String,
    pub profs: Vec<Choice>,
}

