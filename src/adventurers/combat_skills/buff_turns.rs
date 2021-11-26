use crate::enums::{BuffType, HumanReadable, Target};

#[derive(PartialEq, Eq)]
pub struct BuffTurns {
    pub target: Target,
    pub kind: BuffType,
    pub n_turns: i32,
}

impl BuffTurns {
    pub fn new() -> BuffTurns {
        BuffTurns {
            target: Target::Auto,
            kind: BuffType::Buff,
            n_turns: 0,
        }
    }
}

impl HumanReadable for BuffTurns {
    fn to_str(&self) -> String {
        let num_str;
        if self.n_turns < 0 {
            num_str = self.n_turns.to_string();
        } else {
            num_str = format!("+{}", self.n_turns);
        }

        format!(
            "[{}] Status {}{} turns",
            self.target.to_str(),
            self.kind.to_str(),
            num_str
        )
    }
}
