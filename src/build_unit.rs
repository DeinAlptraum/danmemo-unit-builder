use crate::get_attributes::*;
use crate::Unit;

pub fn build_unit() -> Unit {
    let mut unit = Unit::new();
    unit.unit_type = get_unit_type();
    unit.title = get_title();
    unit.name = get_name();
    unit.stars = get_stars();
    unit.limited = get_limited();
    for i in 0..6 {
        let res = get_one_lb_stats(i);
        unit.hp.push(res[0].to_string());
        unit.mp.push(res[1].to_string());
        unit.strength.push(res[2].to_string());
        unit.endurance.push(res[3].to_string());
        unit.dexterity.push(res[4].to_string());
        unit.agility.push(res[5].to_string());
        unit.magic.push(res[6].to_string());
    }

    unit
}
