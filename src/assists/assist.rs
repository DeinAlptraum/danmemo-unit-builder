use crate::AssistSkill;
use crate::InstantSkill;
use crate::Unit;

pub struct Assist {
    pub unit: Unit,
    pub skill: AssistSkill,
    pub instant_skill: Option<InstantSkill>,
}

impl Assist {
    pub fn new(unit: Unit) -> Assist {
        Assist {
            unit: unit,
            skill: AssistSkill::new(),
            instant_skill: None,
        }
    }
}
