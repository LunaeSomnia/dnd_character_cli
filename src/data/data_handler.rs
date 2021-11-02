use std::{fs::File, io::Read};

use super::{CLASSES, RACES, class_data::ClassData, race_data::RaceData};

pub struct DataHandler {
    pub class_data: Vec<ClassData>,
    pub race_data: Vec<RaceData>
}

impl DataHandler {
    pub fn new() -> Self {
        DataHandler {
            class_data: serde_json::from_str(&CLASSES).unwrap(),
            race_data: serde_json::from_str(&RACES).unwrap()
        }
    }
}