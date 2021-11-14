use crate::get_attributes::*;
use crate::Unit;

pub fn build_unit() -> Unit {
    let mut unit = Unit::new();
    unit.unit_type = get_unit_type();
    unit.title = get_title();
    unit.name = get_name();
    unit.stars = get_stars();
    unit.limited = get_limited();
    unit.hp = get_six_stats("HP");
    unit.mp = get_six_stats("MP");
    unit.strength = get_six_stats("Strength");
    unit.endurance = get_six_stats("Endurance");
    unit.dexterity = get_six_stats("Dexterity");
    unit.agility = get_six_stats("Agility");
    unit.magic = get_six_stats("Magic");

    unit
}
