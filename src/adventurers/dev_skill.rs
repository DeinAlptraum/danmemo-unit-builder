use crate::{DevelopmentSkillType, SkillRank};

pub struct DevelopmentSkill {
    pub effect: DevelopmentSkillType,
    pub rank: Option<SkillRank>,
}

impl DevelopmentSkill {
    pub fn new() -> DevelopmentSkill {
        DevelopmentSkill {
            effect: DevelopmentSkillType::Unknown,
            rank: None,
        }
    }
}
