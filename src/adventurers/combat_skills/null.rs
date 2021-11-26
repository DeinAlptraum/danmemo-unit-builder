use crate::enums::{Attribute, HumanReadable, Target};

#[derive(PartialEq, Eq)]
pub struct Null {
    pub target: Target,
    pub kind: Attribute,
    pub amount: u32,
    pub chance: u32,
}

impl Null {
    pub fn new() -> Null {
        Null {
            target: Target::Auto,
            kind: Attribute::NullPhysical,
            amount: 0,
            chance: 0,
        }
    }
}

impl HumanReadable for Null {
    fn to_str(&self) -> String {
        if self.chance < 100 {
            format!(
                "[{}] {}% {} x{}",
                self.target.to_str(),
                self.chance,
                self.kind.to_str(),
                self.amount
            )
        } else {
            format!(
                "[{}] {} x{}",
                self.target.to_str(),
                self.kind.to_str(),
                self.amount
            )
        }
    }
}

impl Null {
    pub fn mod_to_json(&self) -> String {
        let mut res = String::new();
        if self.chance < 100 {
            res.push_str(&format!("{}% ", self.chance));
        }

        res.push_str(&format!("x{}", self.amount));
        res
    }
}
