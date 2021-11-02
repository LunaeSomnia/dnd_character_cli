use serde_derive::{Deserialize, Serialize};
use super::ClassProficiency;


#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct Multiclass {
    pub minimums: Vec<MulticlassRequirement>,
    pub m_type: Option<MulticlassRequirementType>,
    pub gains: Option<Vec<ClassProficiency>>,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct MulticlassRequirement {
    pub stat: String,
    pub minimum: u8,
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub enum MulticlassRequirementType {
    AND,
    OR,
}