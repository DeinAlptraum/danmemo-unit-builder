use crate::adventurer_effects::{Ailment, Buff, BuffRemove, BuffTurns, Damaging, Heal, Null};
use crate::enums::Speed;

/*
 * Possible Adventurer skill effects:
 * Damaging skill, fields: target, element, damage type (type), modifier, speed, attribute: none
 * Temp boost, fields: just enum with type of boost
 * Per skill effect dmg boost, fields: attribute, modifier, target: skill, speed: none
 * Rate buffs: target: skill, target: skill, modifier: ultra
 * HP drain effects? fields: percentage (modifier), target: skill, speed: none
 *
 * element effect just doesn't exist for e.g. old units with element-less attacks
 *
 * Buff/Debuff, fields: target, attribute, duration, modifier, speed
 * Buff/debuff removal, fields: target, attribute, speed
 * Nulls, fields: target, type, modifier (e.g. "50% x1", or without chance if certain)
 * Ailment, fields: target, ailment (attribute), speed
 * Buff turns, fields: target, attribute, duration, speed. For every attribute? Extra enum for "all buffs/all debuffs"??
 * Heals? -> Regen goes under Buff, otherwise fields: target, modifier, attribute, speed
 *      MP heals modifier is %, HP Heal is low/mid/etc. OR % modifier, ???
*/

#[derive(PartialEq, Eq)]
pub struct AdventurerSkill {
    pub name: String,
    pub speed: Speed,
    pub dmg_effect: Option<Damaging>,
    pub buffs: Vec<Buff>,
    pub buff_removals: Vec<BuffRemove>,
    pub buff_turns: Vec<BuffTurns>,
    pub nulls: Vec<Null>,
    pub hp_heal: Option<Heal>,
    pub mp_heal: Option<Heal>,
    pub ailments: Vec<Ailment>,
    pub ailment_cure: bool,
}

impl AdventurerSkill {
    pub fn new() -> AdventurerSkill {
        AdventurerSkill {
            name: String::new(),
            speed: Speed::None,
            dmg_effect: None,
            buffs: Vec::new(),
            buff_removals: Vec::new(),
            buff_turns: Vec::new(),
            nulls: Vec::new(),
            hp_heal: None,
            mp_heal: None,
            ailments: Vec::new(),
            ailment_cure: true,
        }
    }
}
