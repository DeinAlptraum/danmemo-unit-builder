use crate::{RateAttribute, SkillModifier};

#[derive(PartialEq, Eq)]
pub struct RateBuff {
    pub attribute: RateAttribute,
    pub modifier: SkillModifier,
}

impl RateBuff {
    pub fn new() -> RateBuff {
        RateBuff {
            attribute: RateAttribute::Critical,
            modifier: SkillModifier::Low,
        }
    }
}
