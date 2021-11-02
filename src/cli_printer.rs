use colored::{ColoredString, Colorize};

use crate::structures::{character::Character, colors::{Color, ToColor, colorize_string, colorize_string_with_style}, proficiencies::{self, ProficiencyType, Skill}, stats::{Stat, StatType, Stats, mod_to_str}};

pub fn get_name(max_len: usize) -> String {
    let mut name = String::new();
    let mut cool = false;

    while !cool {
        print_question("What is your character name?");

        std::io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");

        let name_len = name.trim_end().len() + 8;
        if name_len < max_len {
            cool = true;
        } else {
            print_error("Your character name is too long.");
            name = String::new();
        }
    }

    String::from(name.trim_end())
}

pub fn get_level() -> u8 {
    let mut entry = String::new();
    let mut level = 0;
    let mut cool = false;

    while !cool {
        print_question("Select your character level: (1-20)");

        std::io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");

        let l: u8 = match entry.trim_end().parse() {
            Ok(x) => x,
            Err(_x) => 0,
        };

        match l {
            0 => print_error("Entry is not a number"),
            _ => level = l
        }

        if level <= 20 && level > 0 {
            cool = true;
        } else {
            entry = String::new();
        }
    }
    level
}

pub fn get_multiclass_allow() -> bool {
    let mut entry = String::new();
    let mut exit = false;
    let mut cool = false;

    while !cool {
        print_question("Allow multiclass? (Yes/No)");

        std::io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line");

        if entry.trim_end() == String::from("Yes") {
            cool = true;
            exit = true;
        } else if entry.trim_end() == String::from("No") {
            cool = true;
            exit = false;
        } else {
            print_error("Entry is not a valid option.");
            entry = String::new();
        }
    }
    exit
}

pub fn print_name(name: &str, ter_width: usize) -> () {
    println!(
        "{}",
        format!(
            "{}{}",
            "  ",
            colorize_string_with_style(&format!(" {} ", name), Color::GreyBG, true, false),
        )
    );
}

pub fn print_sub(sub: &str, ter_width: usize) {
    let spaces = ter_width - sub.len();
    println!(
        "{}",
        format!(
            "{}{}",
            " ",
            colorize_string_with_style(&format!(" {} ", sub), Color::Grey, false, true),
        )
    );
}

pub fn print_base_stats(pb: u8, ac: u8, hp: u8, speed: u8, ter_width: usize) {
    let spaces = ter_width - ac.to_string().len() - hp.to_string().len() - speed.to_string().len() - 24;
    println!(
        "{}{} {}   {} {}   {} {}   {} {}",
        "  ",
        "PB".bold(),
        pb,
        "AC".bold(),
        ac,
        "HP".bold(),
        hp,
        "SPEED".bold(),
        speed
    );
}

pub fn print_stats(ch: &Character, ter_width: usize) {
    let mut up = String::new();
    let mut down = String::new();
    let mut saves = String::new();

    let mut i = 0;

    let stats = &ch.stats;

    for s in 0..stats.stats.len() {
        let statt = StatType::new(s as u8);
        let stat = stats.get(statt).unwrap();
        let v = stat.val_to_str();
        let vmod = stat.mod_to_str();

        let is_prof = ch.proficiencies.get(&ProficiencyType::SavingThrows).unwrap().contains(&statt.to_string());
        let sv_mod = if is_prof {
            colorize_string_with_style(&mod_to_str(stat.to_mod()+ch.pb as i8), statt.to_color(false), true, false)
        } else {
            colorize_string_with_style(&mod_to_str(stat.to_mod() as i8), statt.to_color(false), false, false)

        };

        let sv = if is_prof {
            colorize_string_with_style("★", statt.to_color(false), true, false)
        } else {
            colorize_string_with_style("☆", statt.to_color(false), true, false)
        };

        up.push_str(&format!(" {} ", colorize_string(&format!(" {} ", statt.to_abr()), statt.to_color(true)).bold()));
        down.push_str(&format!(" {} {} ", colorize_string_with_style(&v, Color::Grey, false, true), colorize_string(&vmod, statt.to_color(false))));
        saves.push_str(&format!(" {}{}{}{} ", "[".bright_black(), sv_mod, sv, "]".bright_black()));

        i += 1;
    }

    println!(
        "{}{}\n{}{}\n{}{}",
        " ",
        up,
        " ",
        down,
        " ",
        saves
    )
}

pub fn print_profs(ch: &Character, ter_width: usize) {
    let mut buffer = String::from("    ");
    for skill in ch.proficiencies.get(&ProficiencyType::Skills).unwrap() {
        let x = Skill::from_str(skill);
        let sk = format!("{} {}", x.to_string(), mod_to_str(ch.stats.get(x.to_stat()).unwrap().to_mod()+ch.pb as i8));
        buffer.push_str(&format!("{} ", colorize_string_with_style(&sk, x.to_stat().to_color(false), false, false)));
    }
    for (p, v) in &ch.proficiencies {
        if p.clone() != ProficiencyType::SavingThrows && p.clone() != ProficiencyType::Skills && v.len() > 0 {
            buffer.push_str("\n    ");
            for pv in v {
                buffer.push_str(&format!("{}   ", pv));
            }
        }
    }
    println!("{}", buffer)
}

pub fn print_question(question: &str) -> () {
    println!("{} {}", colorize_string(" ? ", Color::BlueBG), question);
}

pub fn print_error(error: &str) -> () {
    println!(
        "{} {}",
        colorize_string(" ! ", Color::RedBG),
        colorize_string(error, Color::Red)
    );
}

//========================================================================

pub fn print_normal(ch: Character, line_width: usize) -> () {

    print_name(&ch.name, line_width);

    let mut class_str: Vec<String> = vec![];
    for c in &ch.classes {
        class_str.push(format!("{} {} {}", c.sub_class, c.class, c.level));
    }
    let mut str: String = String::new();
    let mut i = 0;
    for c in class_str.iter() {
        if i == 0 {
            str = String::from(class_str[0].clone());
        } else {
            str.push_str(&format!(", {}", c));
        }
        i += 1;
    }
    str.push_str(&format!(", {}", &ch.race.name));
    str.push_str(&format!(", {}", &ch.alignment.to_string()));
    print_sub(&str, line_width);

    println!("");

    print_base_stats(ch.pb, ch.ac_calc(), ch.hp_calc(), ch.speed_calc(), line_width);

    println!("");

    print_stats(&ch, line_width);

    println!("");

    println!("  {}", "Proficiencies:".bold());

    print_profs(&ch, line_width);

    println!("");

    println!("  {}", "Features (from class & race) not included".bright_black());


}