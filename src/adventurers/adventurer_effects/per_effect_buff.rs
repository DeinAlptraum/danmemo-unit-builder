use crate::enums::{Attribute, Target};
use crate::HumanReadable;

#[derive(PartialEq, Eq)]
pub struct PerEffectBuff {
    pub attribute: Attribute,
    pub source: Target,
    pub modifier: u32,
}

impl PerEffectBuff {
    pub fn new() -> PerEffectBuff {
        PerEffectBuff {
            attribute: Attribute::Strength,
            source: Target::Auto,
            modifier: 0,
        }
    }

    pub fn to_str(&self) -> String {
        let mut res = format!("Dmg.+{} per each ", self.modifier);
        if self.source == Target::Auto {
            res.push_str("[Self]");
            res.push_str(&self.attribute.to_str());
            res.push_str("Buff Skill");
        } else {
            res.push_str("Target's");
            res.push_str(&self.attribute.to_str());
            res.push_str("Reduction Skill");
        }

        res
    }

    pub fn to_json(&self) -> String {
        let attr = match self.attribute {
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
            _ => panic!(
                "Match in per_effect_boost was non-exhaustive.
                This shouldn't happen, please report it to the developer."
            ),
        };
        let mut attr = attr.to_string();

        let mut res = "per_each_".to_string();

        if self.source == Target::Auto {
            res.push_str("self_");
            attr.push_str("_buff_skill");
        } else {
            res.push_str("foe_");
            attr.push_str("_debuff_skill");
        }

        res.push_str(&attr);
        res
    }
}
