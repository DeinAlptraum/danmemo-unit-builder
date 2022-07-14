use crate::enums::{Attribute, BuffType, HumanReadable, Target};

#[derive(PartialEq, Eq)]
pub struct BuffRemove {
    pub target: Target,
    pub attribute: Attribute,
    pub kind: BuffType,
}

impl BuffRemove {
    pub fn new() -> BuffRemove {
        BuffRemove {
            target: Target::Auto,
            attribute: Attribute::Strength,
            kind: BuffType::Buff,
        }
    }

    pub fn attr_to_json(&self) -> String {
        match self.attribute {
            Attribute::BuffTurns | Attribute::DebuffTurns => {
                format!("{}_removal_no_assist", self.kind.to_json_long())
            }
            Attribute::HPRegen => {
                format!("{}_removal_no_assist", self.attribute.to_json())
            }
            _ => format!(
                "{}_{}_removal_no_assist",
                self.attribute.to_json(),
                self.kind.to_json()
            ),
        }
    }
}

impl HumanReadable for BuffRemove {
    fn to_str(&self) -> String {
        match self.attribute {
            Attribute::BuffTurns | Attribute::DebuffTurns => format!(
                "[{}] removes Status {}s exc. Assist Skills",
                self.target.to_str(),
                self.kind.to_str()
            ),
            _ => format!(
                "[{}] removes {} {}s exc. Assist Skills",
                self.target.to_str(),
                self.attribute.to_str(),
                self.kind.to_str()
            ),
        }
    }
}
