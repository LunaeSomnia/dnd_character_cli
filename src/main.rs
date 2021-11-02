#![allow(dead_code, unused_variables, unused_imports)]

mod cli_printer;
mod rolls;
mod structures;
mod data;

use std::env;

use terminal_size::{terminal_size, Height, Width};

use structures::{character::Character};
use cli_printer::{get_level, get_multiclass_allow, get_name};

use crate::{cli_printer::print_normal, data::{data_handler::DataHandler}, structures::stats::to_mod};

fn main() {  

    let mut ter_width = 0;
    let mut _ter_height = 0;

    match terminal_size() {
        Some((Width(w), Height(h))) => {
            ter_width = w as usize;
            _ter_height = h as usize;
        }
        None => {}
    }

    let mut arguments = env::args();
    arguments.next();
    for arg in arguments {
        println!("Argument: {}", arg.to_string());
    }

    // ======== 

    let data_handler = DataHandler::new();

    let mut character = Character::new();

    // === INPUTS ===

    // Name input and print

    //character.name = get_name(ter_width);
    character.name = String::from("Test name");
    //let max_level = get_level();
    let max_level = 4;
    //character.multiclassing = get_multiclass_allow();
    character.multiclassing = false;
    
    character.init_classes(&data_handler.class_data, max_level);
    character.init_stats();
    character.init_race(&data_handler.race_data);
    character.init_alignment();

    //print_test_classes(character);

    //character.classes = calculate_classes(character.stats.stats , max_level, multiclass);
    
    print_normal(character, ter_width)
    
}
