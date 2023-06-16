use crate::enums::*;

pub struct Unit {
    pub unit_type: UnitType,
    pub title: String,
    pub name: String,
    pub stars: i32,
    pub limited: LimitedType,
    pub hp: Vec<String>,
    pub mp: Vec<String>,
    pub strength: Vec<String>,
    pub endurance: Vec<String>,
    pub dexterity: Vec<String>,
    pub agility: Vec<String>,
    pub magic: Vec<String>,
}

impl Unit {
    pub fn new() -> Unit {
        Unit {
            unit_type: UnitType::Adventurer,
            title: String::new(),
            name: String::new(),
            stars: 0,
            limited: LimitedType::Regular,
            hp: Vec::new(),
            mp: Vec::new(),
            strength: Vec::new(),
            endurance: Vec::new(),
            dexterity: Vec::new(),
            agility: Vec::new(),
            magic: Vec::new(),
        }
    }
}
