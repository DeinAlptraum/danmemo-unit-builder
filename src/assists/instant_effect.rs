pub struct InstantEffect {
    pub base_duration: u32,
    pub mlb_duration: u32,
    pub max_activations: u32,
}

impl InstantEffect {
    pub fn new() -> InstantEffect {
        InstantEffect {
            base_duration: 0,
            mlb_duration: 0,
            max_activations: 0,
        }
    }
}
