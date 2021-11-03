use crate::enums::*;
use crate::unit::Unit;

pub struct Adventurer {
    pub unit: Unit,
    pub a_type: AttackType,
}

impl Adventurer {
    pub fn new(unit: Unit) -> Adventurer {
        Adventurer {
            unit: unit,
            a_type: AttackType::Balance,
        }
    }
}
