pub mod class_data;
pub mod json;
pub mod race_data;
pub mod data_handler;

pub const CLASSES: &str = r#"
[
    {
        "name": "Barbarian",
        "hit_dice": "D12",
        "subclasses": [
            "Berserker",
            "Totem Warrior"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor",
                    "Medium Armor",
                    "Shields"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Martial Weapons"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Strength",
                    "Constitution"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Animal Handling",
                            "Athletics",
                            "Intimidation",
                            "Nature",
                            "Perception",
                            "Survival"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A greataxe",
                    "Any martial melee weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "Two handaxes",
                    "Any simple weapon"
                ]
            },
            "An explorer's pack and four javelins"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Strength",
                    "minimum": 13
                }
            ],
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Shields"
                    ]
                },
                {
                    "prof_type": "Weapons",
                    "profs": [
                        "Simple Weapons",
                        "Martial Weapons"
                    ]
                }
            ]
        }
    },
    {
        "name": "Bard",
        "hit_dice": "D8",
        "subclasses": [
            "Lore",
            "Valor"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Hand crossbows",
                    "Longswords",
                    "Rapiers",
                    "Shortswords"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": [
                    {
                        "choose": 3,
                        "options": [
                            "@MusicalInstruments"
                        ]
                    }
                ]
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Dexterity",
                    "Charisma"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 3,
                        "options": [
                            "@Skills"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A rapier",
                    "A longsword",
                    "Any simple weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "Diplomat's pack",
                    "Entertainer's pack"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A lute",
                    "Any other musical instrument"
                ]
            },
            "Leather armor and a dagger"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Charisma",
                    "minimum": 13
                }
            ],
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor"
                    ]
                },
                {
                    "prof_type": "Tools",
                    "profs": [
                        {
                            "choose": 1,
                            "options": [
                                "@MusicalInstruments"
                            ]
                        }
                    ]
                },
                {
                    "prof_type": "Skills",
                    "profs": [
                        {
                            "choose": 1,
                            "options": [
                                "@Skills"
                            ]
                        }
                    ]
                }
            ]
        }
    },
    {
        "name": "Cleric",
        "hit_dice": "D8",
        "subclasses": [
            "Knowledge",
            "Life",
            "Light",
            "Nature",
            "Tempest",
            "Trickery",
            "War"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor",
                    "Medium Armor",
                    "Shields"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Wisdom",
                    "Charisma"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "History",
                            "insight",
                            "Medicine",
                            "Persuasion",
                            "Religion"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A mace",
                    "A warhammer @{Prof}"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "Scale mail",
                    "Leather armor",
                    "Chain mail @{Prof}"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A light crossbow and 20 bolts",
                    "Any simple weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A priest's pack",
                    "An explorer's pack"
                ]
            },
            "A shield and a holy sumbol"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Wisdom",
                    "minimum": 13
                }
            ],
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor",
                        "Medium Armor",
                        "Shields"
                    ]
                }
            ]
        }
    },
    {
        "name": "Druid",
        "hit_dice": "D8",
        "subclasses": [
            "Land",
            "Moon"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor",
                    "Medium Armor",
                    "Shields"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Clubs",
                    "Daggers",
                    "Darts",
                    "Javelins",
                    "Maces",
                    "Quarterstaffs",
                    "Scimitars",
                    "Sickles",
                    "Slings",
                    "Spears"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": [
                    "Herbalism kit"
                ]
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Intelligence",
                    "Wisdom"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Arcana",
                            "Animal Handling",
                            "Insight",
                            "Medicine",
                            "Nature",
                            "Perception",
                            "Religion",
                            "Survival"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A wooden shield",
                    "Any simple weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A scimitar",
                    "Any simple melee weapon"
                ]
            },
            "Leather armor, an explorer's pack and a druidic focus"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Wisdom",
                    "minimum": 13
                }
            ],
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor",
                        "Medium Armor",
                        "Shields"
                    ]
                }
            ]
        }
    },
    {
        "name": "Fighter",
        "hit_dice": "D10",
        "subclasses": [
            "Battle Master",
            "Champion",
            "Eldritch Knight"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor",
                    "Medium Armor",
                    "Heavy Armor",
                    "Shields"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Martial Weapons"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Strength",
                    "Constitution"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Acrobatics",
                            "Animal Handling",
                            "Athletics",
                            "History",
                            "Insight",
                            "Intimidation",
                            "Perception",
                            "Survival"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "Chain mail",
                    "Leather armor, longbow and 20 arrows"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A martial weapon and a shield",
                    "Two martial weapons"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A light crossbow and 20 bolts",
                    "Two handaxes"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A dungeoneer's pack",
                    "An explorer's pack"
                ]
            }
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Strength",
                    "minimum": 13
                },
                {
                    "stat": "Dexterity",
                    "minimum": 13
                }
            ],
            "m_type": "OR",
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor",
                        "Medium Armor",
                        "Shields"
                    ]
                },
                {
                    "prof_type": "Weapons",
                    "profs": [
                        "Simple Weapons",
                        "Martial Weapons"
                    ]
                }
            ]
        }
    },
    {
        "name": "Monk",
        "hit_dice": "D8",
        "subclasses": [
            "Four Elements",
            "Open Hand",
            "Shadow"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": []
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Shortswords"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": [
                    {
                        "choose": 1,
                        "options": [
                            "One type of artisan's tools",
                            "Any musical instrument"
                        ]
                    }
                ]
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Strength",
                    "Dexterity"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Acrobatics",
                            "Athletics",
                            "History",
                            "Insight",
                            "Religion",
                            "Stealth"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A shortsword",
                    "Any simple weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A dungeoneer's pack",
                    "An explorer's pack"
                ]
            },
            "10 darts"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Dexterity",
                    "minimum": 13
                },
                {
                    "stat": "Wisdom",
                    "minimum": 13
                }
            ],
            "m_type": "AND",
            "gains": [
                {
                    "prof_type": "Weapons",
                    "profs": [
                        "Simple Weapons",
                        "Shortswords"
                    ]
                }
            ]
        }
    },
    {
        "name": "Paladin",
        "hit_dice": "D10",
        "subclasses": [
            "Ancients",
            "Devotion",
            "Vengeance"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor",
                    "Medium Armor",
                    "Heavy Armor",
                    "Shields"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Martial Weapons"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Wisdom",
                    "Charisma"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Athletics",
                            "Insight",
                            "Intimidation",
                            "Medicine",
                            "Persuasion",
                            "Religion"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A martial weapon and a shield",
                    "Two martial weapons"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "Five javelins",
                    "Any simple melee weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A priest's pack",
                    "An explorer's pack"
                ]
            },
            "Chain mail and a holy symbol"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Strength",
                    "minimum": 13
                },
                {
                    "stat": "Charisma",
                    "minimum": 13
                }
            ],
            "m_type": "AND",
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor",
                        "Medium Armor",
                        "Shields"
                    ]
                },
                {
                    "prof_type": "Weapons",
                    "profs": [
                        "Simple Weapons",
                        "Martial Weapons"
                    ]
                }
            ]
        }
    },
    {
        "name": "Ranger",
        "hit_dice": "D10",
        "subclasses": [
            "Beast Master",
            "Hunter"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor",
                    "Medium Armor",
                    "Shields"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Martial Weapons"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Strength",
                    "Dexterity"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 3,
                        "options": [
                            "Animal Handling",
                            "Athletics",
                            "Insight",
                            "Investigation",
                            "Nature",
                            "Perception",
                            "Stealth",
                            "Survival"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "Scale mail",
                    "Leather armor"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "Two shortswords",
                    "Two simple melee weapons"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A dungeoneer's pack",
                    "An explorer's pack"
                ]
            },
            "A longbow and a quiver of 20 arrows"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Dexterity",
                    "minimum": 13
                },
                {
                    "stat": "Wisdom",
                    "minimum": 13
                }
            ],
            "m_type": "AND",
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor",
                        "Medium Armor",
                        "Shields"
                    ]
                },
                {
                    "prof_type": "Weapons",
                    "profs": [
                        "Simple Weapons",
                        "Martial Weapons"
                    ]
                },
                {
                    "prof_type": "Skills",
                    "profs": [
                        {
                            "choose": 1,
                            "options": [
                                "Animal Handling",
                                "Athletics",
                                "Insight",
                                "Investigation",
                                "Nature",
                                "Perception",
                                "Stealth",
                                "Survival"
                            ]
                        }
                    ]
                }
            ]
        }
    },
    {
        "name": "Rogue",
        "hit_dice": "D8",
        "subclasses": [
            "Arcane Trickster",
            "Assassin",
            "Thief"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons",
                    "Hand crossbows",
                    "Longswords",
                    "Rapiers",
                    "Shortswords"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": [
                    "Thieve's Tools"
                ]
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Dexterity",
                    "Intelligence"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 4,
                        "options": [
                            "Acrobatics",
                            "Athletics",
                            "Deception",
                            "Insight",
                            "Intimidation",
                            "Investigation",
                            "Perception",
                            "Persuasion",
                            "Sleight of Hand",
                            "Stealth"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A rapier",
                    "A shortsword"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A shortbow and quiver of 20 arrows",
                    "A shortsword"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A burglar's pack",
                    "A dungeoneer's pack",
                    "An explorer's pack"
                ]
            },
            "Leather armor, two daggers and thieve's tools"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Dexterity",
                    "minimum": 13
                }
            ],
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor"
                    ]
                },
                {
                    "prof_type": "Tools",
                    "profs": [
                        "Thieve's Tools"
                    ]
                },
                {
                    "prof_type": "Skills",
                    "profs": [
                        {
                            "choose": 1,
                            "options": [
                                "Acrobatics",
                                "Athletics",
                                "Deception",
                                "Insight",
                                "Intimidation",
                                "Investigation",
                                "Perception",
                                "Persuasion",
                                "Sleight of Hand",
                                "Stealth"
                            ]
                        }
                    ]
                }
            ]
        }
    },
    {
        "name": "Sorcerer",
        "hit_dice": "D6",
        "subclasses": [
            "Draconic",
            "Wild"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": []
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Daggers",
                    "Darts",
                    "Slings",
                    "Quarterstaffs",
                    "Light Crossbows"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Constitution",
                    "Charisma"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Arcana",
                            "Deception",
                            "Insight",
                            "Intimidation",
                            "Persuasion",
                            "Religion"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A light crossbow and 20 bolts",
                    "Any simple weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A component pouch",
                    "An arcane focus"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A dungeoneer's pack",
                    "An explorer's pack"
                ]
            },
            "Two daggers"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Charisma",
                    "minimum": 13
                }
            ]
        }
    },
    {
        "name": "Warlock",
        "hit_dice": "D8",
        "subclasses": [
            "Archfey",
            "Fiend",
            "Great Old One"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": [
                    "Light Armor"
                ]
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Simple Weapons"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Wisdom",
                    "Charisma"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Arcana",
                            "Deception",
                            "History",
                            "Intimidation",
                            "Investigation",
                            "Nature",
                            "Religion"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A light crossbow and 20 bolts",
                    "Any simple weapon"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A component pouch",
                    "An arcane focus"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A scholar's pack",
                    "A dungeoneer's pack"
                ]
            },
            "Leather armor, any simple weapon and two daggers"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Charisma",
                    "minimum": 13
                }
            ],
            "gains": [
                {
                    "prof_type": "Armor",
                    "profs": [
                        "Light Armor"
                    ]
                },
                {
                    "prof_type": "Weapons",
                    "profs": [
                        "Simple Weapons"
                    ]
                }
            ]
        }
    },
    {
        "name": "Wizard",
        "hit_dice": "D6",
        "subclasses": [
            "Abjuration",
            "Conjuration",
            "Divination",
            "Enchantment",
            "Evocation",
            "Illusion",
            "Necromancy",
            "Transmutation"
        ],
        "proficiencies": [
            {
                "prof_type": "Armor",
                "profs": []
            },
            {
                "prof_type": "Weapons",
                "profs": [
                    "Daggers",
                    "Darts",
                    "Slings",
                    "Quarterstaffs",
                    "Light Crossbows"
                ]
            },
            {
                "prof_type": "Tools",
                "profs": []
            },
            {
                "prof_type": "Saving Throws",
                "profs": [
                    "Intelligence",
                    "Wisdom"
                ]
            },
            {
                "prof_type": "Skills",
                "profs": [
                    {
                        "choose": 2,
                        "options": [
                            "Arcana",
                            "History",
                            "Insight",
                            "Investigation",
                            "Medicine",
                            "Religion"
                        ]
                    }
                ]
            }
        ],
        "equipment": [
            {
                "choose": 1,
                "options": [
                    "A quarterstaff",
                    "A dagger"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A component pouch",
                    "An arcane focus"
                ]
            },
            {
                "choose": 1,
                "options": [
                    "A scholar's pack",
                    "An explorer's pack"
                ]
            },
            "A spellbook"
        ],
        "multiclassing": {
            "minimums": [
                {
                    "stat": "Intelligence",
                    "minimum": 13
                }
            ]
        }
    }
]
"#;

pub const RACES: &str = r#"
[
    {
        "name": "Dragonborn",
        "speed": 30,
        "traits": [
            {
                "stat": "Strength",
                "value": 2
            },
            {
                "stat": "Charisma",
                "value": 1
            },
            {
                "name": "Draconic Ancestry"
            },
            {
                "name": "Breath Weapon"
            },
            {
                "name": "Damage Resistance"
            }
        ]
    },
    {
        "name": "Dwarf",
        "speed": 25,
        "traits": [
            {
                "stat": "Constitution",
                "value": 2
            },
            {
                "name": "Darkvision"
            },
            {
                "name": "Dawrven Resilience"
            },
            {
                "name": "Dwarven Combat Training"
            },
            {
                "name": "Stonecunning"
            }
        ]
    },
    {
        "name": "Elf",
        "speed": 30,
        "traits": [
            {
                "stat": "Dexterity",
                "value": 2
            },
            {
                "name": "Darkvision"
            },
            {
                "name": "Keen Senses"
            },
            {
                "name": "Fey Ancestry"
            },
            {
                "name": "Trance"
            }
        ]
    },
    {
        "name": "Gnome",
        "speed": 25,
        "traits": [
            {
                "stat": "Intelligence",
                "value": 2
            },
            {
                "name": "Darkvision"
            },
            {
                "name": "Gnome Cunning"
            }
        ]
    },
    {
        "name": "Half-Elf",
        "speed": 30,
        "traits": [
            {
                "stat": "Charisma",
                "value": 2
            },
            {
                "choice": {
                    "choose": 2,
                    "options": [
                        "Strength",
                        "Dexterity",
                        "Constitution",
                        "Intelligence",
                        "Wisdom"
                    ]
                },
                "value": 1
            },
            {
                "name": "Darkvision"
            },
            {
                "name": "Fey Ancestry"
            },
            {
                "name": "Skill Versatility"
            }
        ]
    },
    {
        "name": "Half-Orc",
        "speed": 30,
        "traits": [
            {
                "stat": "Strength",
                "value": 2
            },
            {
                "stat": "Constitution",
                "value": 1
            },
            {
                "name": "Darkvision"
            },
            {
                "name": "Menacing"
            },
            {
                "name": "Relentless Endurance"
            },
            {
                "name": "Savage Attacks"
            }
        ]
    },
    {
        "name": "Halfling",
        "speed": 25,
        "traits": [
            {
                "stat": "Dexterity",
                "value": 2
            },
            {
                "name": "Lucky"
            },
            {
                "name": "Brave"
            },
            {
                "name": "Halfling Nimbleness"
            }
        ]
    },
    {
        "name": "Human",
        "speed": 30,
        "traits": [
            {
                "stat": "Strength",
                "value": 1
            },
            {
                "stat": "Dexterity",
                "value": 1
            },
            {
                "stat": "Constitution",
                "value": 1
            },
            {
                "stat": "Intelligence",
                "value": 1
            },
            {
                "stat": "Wisdom",
                "value": 1
            },
            {
                "stat": "Charisma",
                "value": 1
            },
            {
                "name": "Extra Language"
            }
        ]
    },
    {
        "name": "Tiefling",
        "speed": 30,
        "traits": [
            {
                "stat": "Charisma",
                "value": 2
            },
            {
                "stat": "Intelligence",
                "value": 1
            },
            {
                "name": "Darkvision"
            },
            {
                "name": "Hellish Resistance"
            },
            {
                "name": "Infernal Legacy"
            }
        ]
    }
]
"#;