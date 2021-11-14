use crate::AssistEffect;

pub struct AssistSkill {
    pub name: String,
    pub base_effects: Vec<AssistEffect>,
    pub mlb_effects: Vec<AssistEffect>,
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
