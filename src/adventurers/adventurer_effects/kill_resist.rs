use crate::Target;

#[derive(PartialEq, Eq)]
pub struct KillResist {
    pub target: Target,
    pub threshold: u32,
}

impl KillResist {
    pub fn new() -> KillResist {
        KillResist {
            target: Target::Auto,
            threshold: 0,
        }
    }
}
