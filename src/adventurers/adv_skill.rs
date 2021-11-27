use crate::combat_skills::{
    Ailment, Buff, BuffRemove, BuffTurns, Damaging, Heal, KillResist, Null,
};
use crate::enums::Speed;

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
    pub additional_action: Option<u32>,
    pub ailments: Vec<Ailment>,
    pub ailment_cure: bool,
    pub kill_resist: Option<KillResist>,
}

impl AdventurerSkill {
    pub fn new() -> AdventurerSkill {
        AdventurerSkill {
            name: String::from(""),
            speed: Speed::None,
            dmg_effect: None,
            buffs: Vec::new(),
            buff_removals: Vec::new(),
            buff_turns: Vec::new(),
            nulls: Vec::new(),
            hp_heal: None,
            mp_heal: None,
            additional_action: None,
            ailments: Vec::new(),
            ailment_cure: false,
            kill_resist: None,
        }
    }
}
