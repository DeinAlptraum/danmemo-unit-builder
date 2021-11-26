use crate::enums::{Attribute, HealModifier, Target};

#[derive(PartialEq, Eq)]
pub struct Heal {
    pub target: Target,
    pub heal_type: Attribute,
    pub modifier: HealModifier,
}

impl Heal {
    pub fn new() -> Heal {
        Heal {
            target: Target::Auto,
            heal_type: Attribute::Heal,
            modifier: HealModifier::Numeric(0),
        }
    }
}
