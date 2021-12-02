use crate::{enums::HumanReadable, enums::SkillRank, DevelopmentSkillType};

#[derive(PartialEq, Eq)]
pub struct DevelopmentSkill {
    pub effect: DevelopmentSkillType,
    pub rank: Option<SkillRank>,
}

impl HumanReadable for DevelopmentSkill {
    fn to_str(&self) -> String {
        let mut res = self.effect.to_str();
        if let Some(rk) = &self.rank {
            res.push_str(": ");
            res.push_str(&rk.to_str());
        }
        res
    }
}

impl DevelopmentSkill {
    pub fn new() -> DevelopmentSkill {
        DevelopmentSkill {
            effect: DevelopmentSkillType::Unknown,
            rank: None,
        }
    }
}
