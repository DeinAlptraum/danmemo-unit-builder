use crate::enums::{Attribute, HumanReadable, Target};
use crate::AssistEffect;

#[derive(PartialEq, Eq)]
pub struct Buff {
    pub target: Target,
    pub attribute: Attribute,
    pub modifier: i32,
    pub duration: u32,
}

impl Buff {
    pub fn new() -> Buff {
        Buff {
            target: Target::Auto,
            attribute: Attribute::Strength,
            modifier: 0,
            duration: 0,
        }
    }
}

impl HumanReadable for Buff {
    fn to_str(&self) -> String {
        let mut ass_ef = AssistEffect::new();
        ass_ef.target = self.target.clone();
        ass_ef.attribute = self.attribute.clone();
        ass_ef.modifier = self.modifier.clone();

        let mut res = ass_ef.to_str();
        let dur = &format!(" /{} turn(s)", self.duration);
        res.push_str(dur);
        res
    }
}
