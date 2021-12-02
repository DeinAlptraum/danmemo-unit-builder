use crate::{enums::DamageType, AdventurerSkill};

use crate::Element;
pub struct InstantSkill {
    pub effects: AdventurerSkill,
    pub damage_type: Option<DamageType>,
    pub element: Option<Element>,
}

impl InstantSkill {
    pub fn new() -> InstantSkill {
        InstantSkill {
            effects: AdventurerSkill::new(),
            damage_type: None,
            element: None,
        }
    }
}
