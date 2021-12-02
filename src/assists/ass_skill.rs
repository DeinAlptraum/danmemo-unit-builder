use crate::{AssistEffect, InstantEffect};

pub struct AssistSkill {
    pub name: String,
    pub base_effects: Vec<AssistEffect>,
    pub mlb_effects: Vec<AssistEffect>,
    pub instant_effect: Option<InstantEffect>,
}

impl AssistSkill {
    pub fn new() -> AssistSkill {
        AssistSkill {
            name: String::new(),
            base_effects: Vec::new(),
            mlb_effects: Vec::new(),
            instant_effect: None,
        }
    }
}
