#[derive(PartialEq, Eq)]
pub struct AdditionalAction {
    pub quantity: u32,
    pub effect: String,
}

impl AdditionalAction {
    pub fn new() -> AdditionalAction {
        AdditionalAction {
            quantity: 0,
            effect: String::new(),
        }
    }
}
