use crate::enums::{Attribute, Target};
use crate::HumanReadable;

#[derive(PartialEq, Eq, Clone)]
pub struct AssistEffect {
    pub target: Target,
    pub attribute: Attribute,
    pub modifier: i32,
}

impl AssistEffect {
    pub fn new() -> AssistEffect {
        AssistEffect {
            attribute: Attribute::Strength,
            target: Target::Auto,
            modifier: 0,
        }
    }

    fn mod_to_signed(&self) -> String {
        let modi = match self.attribute {
            Attribute::AoEResistance | Attribute::STResistance => self.modifier * -1,
            _ => self.modifier,
        };
        if modi >= 0 {
            format!("+{}", modi)
        } else {
            format!("{}", modi)
        }
    }

    pub fn mod_to_str(&self) -> String {
        match self.attribute {
            Attribute::NullPhysical | Attribute::NullMagical | Attribute::NullAilment => {
                format!("x{}", self.modifier)
            }
            Attribute::Heal | Attribute::MPRegen | Attribute::HPRegen => self.modifier.to_string(),
            _ => self.mod_to_signed(),
        }
    }

    pub fn to_str(&self) -> String {
        let res = match self.attribute {
            Attribute::AoEResistance | Attribute::STResistance => {
                format!("{}{}%", self.attribute.to_str(), self.mod_to_str())
            }
            Attribute::Heal => {
                format!("{}% {}", self.modifier, self.attribute.to_str())
            }
            Attribute::MPRegen => {
                format!("{} {}", self.modifier, self.attribute.to_str())
            }
            Attribute::HPRegen => {
                format!("{}% {}", self.modifier, self.attribute.to_str())
            }
            Attribute::Sleep
            | Attribute::Stun
            | Attribute::Seal
            | Attribute::Slow
            | Attribute::Taunt
            | Attribute::Poison => {
                format!("{}% {}", self.modifier, self.attribute.to_str())
            }
            Attribute::NullPhysical | Attribute::NullMagical | Attribute::NullAilment => {
                format!("{} x{}", self.attribute.to_str(), self.modifier)
            }
            Attribute::BuffTurns | Attribute::DebuffTurns => {
                format!("{} {} turns", self.attribute.to_str(), self.mod_to_signed())
            }
            Attribute::SACharge => {
                format!("Increase {} by {}%", self.attribute.to_str(), self.modifier)
            }
            _ => {
                format!("{}{}%", self.attribute.to_str(), self.mod_to_signed())
            }
        };
        let res = format!("[{}] {}", self.target.to_str(), res);
        res
    }
}
