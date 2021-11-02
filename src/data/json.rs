use serde_derive::{Deserialize, Serialize};

use crate::rolls;

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Choice {
    String(String),
    Choose(Choose),
}

#[derive(Eq, PartialEq, Deserialize, Serialize, Debug)]
pub struct Choose {
    pub choose: u8,
    pub options: Vec<String>
}

impl Choice {
    pub fn choose(&self) -> Vec<String> {
        match &self {
            Choice::String(x) => vec![x.clone()],
            Choice::Choose(x) => {
                let mut left = x.choose;
                let mut chosen: Vec<String> = vec![];
                while left != 0 {
                    let n = rolls::random(0, x.options.len());
                    if !chosen.contains(&x.options[n]) {
                        left -= 1;
                        chosen.push(x.options[n].clone());
                    }
                }
                chosen
            }
            
        }
    }
}
