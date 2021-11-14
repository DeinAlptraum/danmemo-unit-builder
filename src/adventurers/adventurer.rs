use crate::enums::*;
use crate::{AdventurerSkill, Unit};

pub struct Adventurer {
    pub unit: Unit,
    pub adventurer_type: AdventurerType,
    pub damage_type: DamageType,
    pub element: Element,
    pub sa: AdventurerSkill,
    pub reg_skills: Vec<AdventurerSkill>,
}

impl Adventurer {
    pub fn new(unit: Unit) -> Adventurer {
        Adventurer {
            unit: unit,
            adventurer_type: AdventurerType::Balance,
            damage_type: DamageType::Physical,
            element: Element::Fire,
            sa: AdventurerSkill::new(),
            reg_skills: Vec::new(),
        }
    }
}