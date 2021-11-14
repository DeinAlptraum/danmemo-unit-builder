use super::per_effect_buff::PerEffectBuff;
use crate::enums::{RateAttribute, SkillModifier, Target, TempBoost};

#[derive(PartialEq, Eq)]
pub struct Damaging {
    pub target: Target,
    pub dmg_mod: SkillModifier,
    pub temp_boost: Option<TempBoost>,
    pub rate_buff: Option<RateAttribute>,
    pub lifesteal: Option<u32>,
    pub per_effect_buffs: Vec<PerEffectBuff>,
}

impl Damaging {
    pub fn new() -> Damaging {
        Damaging {
            target: Target::Auto,
            dmg_mod: SkillModifier::Low,
            temp_boost: None,
            rate_buff: None,
            lifesteal: None,
            per_effect_buffs: Vec::new(),
        }
    }
}
