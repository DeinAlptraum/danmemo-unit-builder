use crate::effect::Effect;

pub struct AssistSkill {
    pub name: String,
    pub base_effects: Vec<Effect>,
    pub mlb_effects: Vec<Effect>,
}

impl AssistSkill {
    pub fn new() -> AssistSkill {
        AssistSkill {
            name: String::new(),
            base_effects: Vec::new(),
            mlb_effects: Vec::new(),
        }
    }
}
