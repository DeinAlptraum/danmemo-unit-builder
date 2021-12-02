use std::fs::File;
use std::io::Write;

mod json_strings;

mod enums;
use crate::enums::*;

mod unit;
pub use unit::Unit;

mod adventurers;
pub use adventurers::*;

mod assists;
pub use assists::*;

mod get_attributes;

mod build_unit;
use crate::build_unit::*;

mod gen_json;
pub use gen_json::*;

fn main() {
    println!("Before you start, please note:
This program operates under the following simplifying assumptions:
    Every unit uses either physical or magical attacks of one element, and all its attack skills have that damage type and element
    A phys type/magic type unit only has phys type/magic type attacks
    per_effect type boosts for ailments don't exist
    Buff/Debuff turns affecting skills exist only for all effect (e.g. no '[Foes] Strength Buffs -1 turn')
    Ailment cure always affects [Allies] (not true, e.g. Elven Awakening Lefiya affects Self)
    Revival skills (e.g. Forbidden Chant Fels' SA) don't exist
    Curse removal skills (e.g. Cynic Cassandra Ilion) don't exist
    attacks indexed to certain attributes don't exist
Should any of these assumptions be incorrect for the unit you're creating, you will have to fix the resulting JSON yourself.\n");

    let unit = build_unit();

    if unit.unit_type == UnitType::Assist {
        write_ass(unit);
    } else {
        write_adv(unit);
    }
}

// Helper functions
fn wri(mut file: &File, text: &str) {
    file.write(text.as_bytes()).unwrap();
}

fn file_name_from_unit(unit: &Unit) -> String {
    let mut file_name = String::new();
    file_name.push_str(&unit.title);
    file_name.push_str(" - ");
    file_name.push_str(&unit.name);
    file_name.push_str(".json");

    file_name
}

pub fn write_ass(unit: Unit) {
    let ass = build_ass(unit);

    let file_name = file_name_from_unit(&ass.unit);
    let file = File::create(file_name).unwrap();

    let header = gen_ass_header(&ass);
    wri(&file, &header);

    let stats = gen_stats(&ass.unit);
    wri(&file, &stats);

    let ass_skills = gen_ass_skill(&ass);
    wri(&file, &ass_skills);
}

pub fn write_adv(unit: Unit) {
    let adv = build_adv(unit);

    let file_name = file_name_from_unit(&adv.unit);
    let file = File::create(file_name).unwrap();

    let header = gen_adv_header(&adv);
    wri(&file, &header);

    let stats = gen_stats(&adv.unit);
    wri(&file, &stats);

    let sa = gen_sa(&adv);
    wri(&file, &sa);

    let skills = gen_adv_skills(&adv);
    wri(&file, &skills);

    let aa = gen_aa(&adv);
    wri(&file, &aa);

    let dev_skills = gen_dev_skills(&adv);
    wri(&file, &dev_skills);
}
