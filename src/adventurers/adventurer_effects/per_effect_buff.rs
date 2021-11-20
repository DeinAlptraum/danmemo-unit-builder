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
        let mut res = String::from("per_each_");

        if self.source == Target::Auto {
            res.push_str("self_");
        } else {
            res.push_str("targets_");
        }

        let attr = match attr {
            // isn't always equal to the pure attribute json strings we get from Attribute.to_json()
            Attribute::Strength => "str",
            Attribute::Endurance => "end",
            Attribute::Dexterity => "dex",
            Attribute::Agility => "agi",
            Attribute::Magic => "mag",
            Attribute::PhysicalResistance => "physical_resist",
            Attribute::MagicResistance => "magic_resist",
            Attribute::AoEResistance => "all_targets_damage_resist",
            Attribute::STResistance => "single_target_damage_resist",
            Attribute::LightDamage => "light_attack",
            Attribute::DarkDamage => "dark_attack",
            Attribute::FireDamage => "fire_attack",
            Attribute::WaterDamage => "water_attack",
            Attribute::ThunderDamage => "thunder_attack",
            Attribute::EarthDamage => "earth_attack",
            Attribute::WindDamage => "wind_attack",
            Attribute::GuardRate => "guard_rate",
            Attribute::CounterRate => "counter_rate",
            Attribute::CriticalRate => "critical_rate",
            Attribute::PenetrationRate => "penetration_rate",
            Attribute::StrengthAndMagic => "strength_and_magic",
            _ => panic!(
                "Match in per_effect_boost was non-exhaustive.
                This shouldn't happen, please report it to the developer."
            ),
        };

        res.push_str(attr);

        if self.kind == BuffType::Buff {
            res.push_str("_buff_skill");
        } else {
            res.push_str("_reduction_skill");
        }

        res
    }
}
