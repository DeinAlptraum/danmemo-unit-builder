use super::{per_effect_buff::PerEffectBuff, RateBuff};
use crate::enums::{SkillModifier, Target, TempBoost};

#[derive(PartialEq, Eq)]
pub struct Damaging {
    pub target: Target,
    pub dmg_mod: SkillModifier,
    pub temp_boost: Option<TempBoost>,
    pub rate_buff: Option<RateBuff>,
    pub lifesteal: Option<u32>,
    pub per_effect_boost: Option<PerEffectBuff>,
}

impl Damaging {
    pub fn new() -> Damaging {
        Damaging {
            target: Target::Auto,
            dmg_mod: SkillModifier::Low,
            temp_boost: None,
            rate_buff: None,
            lifesteal: None,
            per_effect_boost: None,
        }
    }
}
