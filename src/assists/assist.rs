use crate::AssistSkill;
use crate::Unit;

pub struct Assist {
    pub unit: Unit,
    pub skill: AssistSkill,
}

impl Assist {
    pub fn new(unit: Unit) -> Assist {
        Assist {
            unit: unit,
            skill: AssistSkill::new(),
        }
    }
}
