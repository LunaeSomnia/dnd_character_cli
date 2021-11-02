use super::stats::StatType;
pub enum ModifierType {
    None,
    Prof,
    Expertise
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ProficiencyType {
    Armor,
    Weapons,
    Tools,
    SavingThrows,
    Skills
}

impl ToString for ProficiencyType {
    fn to_string(&self) -> String {
        match &self {
            ProficiencyType::Armor => String::from("Armor"),
            ProficiencyType::Weapons => String::from("Weapons"),
            ProficiencyType::Tools => String::from("Tools"),
            ProficiencyType::SavingThrows => String::from("Saving Throws"),
            ProficiencyType::Skills => String::from("Skills")
        }
    }
}

impl ProficiencyType {
    pub fn from_string(s: &str) -> Option<ProficiencyType> {
        match s {
            "Armor" => Some(ProficiencyType::Armor),
            "Weapons" => Some(ProficiencyType::Weapons),
            "Tools" => Some(ProficiencyType::Tools),
            "Saving Throws" => Some(ProficiencyType::SavingThrows),
            "Skills" => Some(ProficiencyType::Skills),
            _ => None
        }
    }
}

pub struct SkillProficiency {
    pub prof_type: StatType,
    pub prof_mod: ModifierType
}

#[derive(Clone, Copy)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival
}

impl Skill {
    pub fn to_stat(&self) -> StatType {
        match &self {
            Skill::Acrobatics => StatType::Dexterity,
            Skill::AnimalHandling => StatType::Wisdom,
            Skill::Arcana => StatType::Intelligence,
            Skill::Athletics => StatType::Strength,
            Skill::Deception => StatType::Charisma,
            Skill::History => StatType::Intelligence,
            Skill::Insight => StatType::Wisdom,
            Skill::Intimidation => StatType::Charisma,
            Skill::Investigation => StatType::Intelligence,
            Skill::Medicine => StatType::Wisdom,
            Skill::Nature => StatType::Intelligence,
            Skill::Perception => StatType::Wisdom,
            Skill::Performance => StatType::Charisma,
            Skill::Persuasion => StatType::Charisma,
            Skill::Religion => StatType::Intelligence,
            Skill::SleightOfHand => StatType::Dexterity,
            Skill::Stealth => StatType::Dexterity,
            Skill::Survival => StatType::Wisdom
        }
    }

    pub fn from_str(str: &str) -> Skill {
        match str {
            "Acrobatics" => Skill::Acrobatics,
            "Animal Handling" => Skill::AnimalHandling,
            "Arcana" => Skill::Arcana,
            "Athletics" => Skill::Athletics,
            "Deception" => Skill::Deception,
            "History" => Skill::History,
            "Insight" => Skill::Insight,
            "Intimidation" => Skill::Intimidation,
            "Investigation" => Skill::Investigation,
            "Medicine" => Skill::Medicine,
            "Nature" => Skill::Nature,
            "Perception" => Skill::Perception,
            "Performance" => Skill::Performance,
            "Persuasion" => Skill::Persuasion,
            "Religion" => Skill::Religion,
            "Sleight Of Hand" => Skill::SleightOfHand,
            "Stealth" => Skill::Stealth,
            _ => Skill::Survival
        }
    }
}

impl ToString for Skill {
    fn to_string(&self) -> String {
        match &self {
            Skill::Acrobatics => String::from("Acrobatics"),
            Skill::AnimalHandling => String::from("Animal Handling"),
            Skill::Arcana => String::from("Arcana"),
            Skill::Athletics => String::from("Athletics"),
            Skill::Deception => String::from("Deception"),
            Skill::History => String::from("History"),
            Skill::Insight => String::from("Insight"),
            Skill::Intimidation => String::from("Intimidation"),
            Skill::Investigation => String::from("Investigation"),
            Skill::Medicine => String::from("Medicine"),
            Skill::Nature => String::from("Nature"),
            Skill::Perception => String::from("Perception"),
            Skill::Performance => String::from("Performance"),
            Skill::Persuasion => String::from("Persuasion"),
            Skill::Religion => String::from("Religion"),
            Skill::SleightOfHand => String::from("Sleight of Hand"),
            Skill::Stealth => String::from("Stealth"),
            Skill::Survival => String::from("Survival")
        }
    }
}

