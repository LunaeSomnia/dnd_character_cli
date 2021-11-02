use std::{collections::HashMap, vec};

use crate::{data::{class_data::ClassData, race_data::{RaceData, TraitOption}}, rolls::{random, roll_stat}};

use super::{alignment::Alignment, class::Class, proficiencies::ProficiencyType, stats::{Stat, StatType, Stats}};

pub struct Character {
    pub name: String,
    pub multiclassing: bool,
    pub classes: Vec<Class>,
    pub pb: u8,
    pub race: Race,
    pub alignment: Alignment,

    pub stats: Stats,
    pub proficiencies: HashMap<ProficiencyType, Vec<String>>,
    pub equipment: Vec<String>,
    pub traits: Vec<String>,
}

pub struct Race {
    pub name: String,
    pub speed: u8
}

impl Character {
    pub fn new() -> Character {

        let mut profs: HashMap<ProficiencyType, Vec<String>> = HashMap::new();

        profs.insert(ProficiencyType::Armor, vec![]);
        profs.insert(ProficiencyType::Weapons, vec![]);
        profs.insert(ProficiencyType::Tools, vec![]);
        profs.insert(ProficiencyType::SavingThrows, vec![]);
        profs.insert(ProficiencyType::Skills, vec![]);

        Character {
            name: String::new(),
            multiclassing: false,
            classes: vec![],
            pb: 0,
            race: Race{name:String::new(), speed: 0},
            alignment: Alignment::Neutral,
            stats: Stats::new(),
            proficiencies: profs,
            equipment: vec![],
            traits: vec![],
        }
    }

    pub fn max_level(&self) -> u8 {
        let mut sum: u8 = 0;
        for c in &self.classes {
            sum += c.level;
        }
        sum
    }

    pub fn add_class(&mut self, class: Class) {
        let mut is = false;
        for c in &self.classes {
            if c.class == class.class {
                is = true;
            }
        }
        if !is {
            self.classes.push(class)
        }
    }

    pub fn add_stat(&mut self, stat: Stat) {
        todo!();
    }

    pub fn add_prof(&mut self, prof_type: ProficiencyType, prof: String) {

    }

    pub fn init_classes(&mut self, classes: &Vec<ClassData>, max_level: u8) {
        if self.multiclassing {
            todo!();
        } else {
            let selected_class = &classes[random(0, classes.len())];

            self.classes = vec![Class::from_data(selected_class, max_level)];

            for p in &selected_class.proficiencies {

                let arr= self.proficiencies.get_mut(&ProficiencyType::from_string(&p.prof_type)
                    .expect(&format!("No proficiency found for '{}'", p.prof_type)))
                    .expect(&format!("Value not found in map for prof '{}'", p.prof_type));
                for p in &p.profs {
                    let c = p.choose();
                    for l in c {
                        if !arr.contains(&l) {
                            arr.push(l.clone());
                        }
                    }
                }
            }
        }

        self.pb = self.pb_calc();
    }

    pub fn init_race(&mut self, races: &Vec<RaceData>) {
        let selected_race = &races[random(0, races.len())];

        self.race = Race { name: selected_race.name.clone(), speed: selected_race.speed }; //@TODO
        for t in &selected_race.traits {
            match t {
                TraitOption::Trait(x) => {
                    self.traits.push(x.name.clone());
                },
                TraitOption::Stat(x) => {
                    self.stats.sum(StatType::from_str(&x.stat).expect(&format!("StatType not included: '{}'", x.stat)), x.value)
                },
                TraitOption::StatOption(x) => {
                    let chosen = x.choice.choose();
                    for c in chosen {
                        self.stats.sum(StatType::from_str(&c).expect(&format!("StatType not included: '{}'", c)), x.value)
                    }
                }
            }
        }
    }

    pub fn init_alignment(&mut self) {
        self.alignment = Alignment::new(random(0, 9));
    }

    pub fn pb_calc(&self) -> u8 {
        let lvl = self.max_level();
        if lvl != 0 {
            if lvl < 5 {
                2
            } else if lvl < 9 {
                3
            } else if lvl < 13 {
                4
            } else if lvl < 17 {
                5
            } else {
                6
            }
        } else {
            0
        }
    }

    pub fn ac_calc(&self) -> u8 {
        self.stats.get(StatType::Dexterity).unwrap().value
    }

    pub fn hp_calc(&self) -> u8 {
        let con: i16 = self.stats.get(StatType::Constitution).unwrap().to_mod() as i16;
        let mut ci = 0;
        let mut sum: i16 = self.classes[0].hit_dice.to_num() as i16 + con;
        for c in self.classes.iter().next() {
            if ci == 0 {
                sum += c.hit_dice.to_num() as i16 + (c.level as i16-1)*(c.hit_dice.to_num()as i16/2+1+con)
            } else {
                sum += c.level as i16 * (c.hit_dice.to_num() as i16/2+1+con);
            }
            ci += 1;
        }
        sum as u8
    }

    pub fn init_stats(&mut self) {
        let mut rolls = vec![
            roll_stat(),
            roll_stat(),
            roll_stat(),
            roll_stat(),
            roll_stat(),
            roll_stat()
        ];

        rolls.sort_by(|a, b| b.cmp(a)); // Sort greater first

        let savs = self.proficiencies.get(&ProficiencyType::SavingThrows).unwrap();

        let mut r = rolls.iter();

        for s in savs {
            let val = r.next().unwrap().clone();
            let i = self.stats.get_i(StatType::from_str(s).unwrap());
            self.stats.stats[i].value = val;
        }

        for s in &mut self.stats.stats {
            if s.value == 0 {
                let x = r.next().unwrap().clone();
                s.value = x;
            }
        }
    }

    pub fn speed_calc(&self) -> u8 {
        self.race.speed
    }
}