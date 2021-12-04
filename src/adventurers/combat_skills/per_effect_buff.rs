use crate::enums::{Attribute, BuffType, Target};
use crate::HumanReadable;

#[derive(PartialEq, Eq)]
pub struct PerEffectBuff {
    pub attributes: Vec<Attribute>,
    pub source: Target,
    pub kind: BuffType,
    pub modifier: u32,
}

impl HumanReadable for PerEffectBuff {
    fn to_str(&self) -> String {
        let mut res = format!("Dmg.+{}% per each ", self.modifier);
        if self.source == Target::Auto {
            res.push_str("[Self] ");
        } else {
            res.push_str("Target's ");
        }

        for attr in &self.attributes {
            res.push_str(&attr.to_str());
            if attr != self.attributes.last().unwrap() {
                res.push_str(", ");
            }
        }

        if self.kind == BuffType::Buff {
            res.push_str(" Buff Skill");
        } else {
            res.push_str(" Reduction Skill");
        }

        res
    }
}

impl PerEffectBuff {
    pub fn new() -> PerEffectBuff {
        PerEffectBuff {
            attributes: Vec::new(),
            source: Target::Auto,
            kind: BuffType::Buff,
            modifier: 0,
        }
    }

    pub fn to_json(&self, attr: &Attribute) -> String {
        let mut res: String;
        match attr {
            Attribute::Sleep
            | Attribute::Stun
            | Attribute::Seal
            | Attribute::Slow
            | Attribute::Taunt
            | Attribute::Poison => {
                res = String::from("against_");
                res.push_str(attr.to_json());
                res.push_str("_target");
            }
            _ => {
                res = String::from("per_each_");

                if self.source == Target::Auto {
                    res.push_str("self_");
                } else {
                    res.push_str("target_");
                }

                res.push_str(attr.to_json());
                res.push_str("_");
                res.push_str(&self.kind.to_json());
            }
        }

        res
    }
}
