use crate::enums::{Attribute, HumanReadable, Target};

#[derive(PartialEq, Eq)]
pub struct Ailment {
    pub target: Target,
    pub kind: Attribute,
    pub chance: u32,
}

impl Ailment {
    pub fn new() -> Ailment {
        Ailment {
            target: Target::Auto,
            kind: Attribute::Sleep,
            chance: 0,
        }
    }
}

impl HumanReadable for Ailment {
    fn to_str(&self) -> String {
        format!(
            "[{}] {}% {}",
            self.target.to_str(),
            self.chance,
            self.kind.to_str(),
        )
    }
}
