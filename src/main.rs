use regex::{Captures, Regex};
use std::fs::File;
use std::io::Write;

mod json_strings;
use crate::json_strings::*;

mod enums;
use crate::enums::*;

mod get_attributes;
use crate::get_attributes::*;
mod build_obj;
use crate::build_obj::*;

mod objects;
use crate::objects::{Unit,Assist};
//use crate::adventurer::Adventurer;
//use crate::effect::Effect;
//use crate::ass_skill::AssistSkill;

/* To-Dos:

 * to_string() Trait for enums & delete module crate::enum_to_str?
 * better attackType / skillType distinction??
 *
 * effect struct
 * ass-skil & adv-skill struct
*/

fn main() {
    let unit = build_unit();

    if unit.u_type == UnitType::Assist {
        gen_ass(unit);
    } else {
        gen_adv(unit);
    }
}

// Copied from StacKOverflow
fn template_replace(template: &str, values: &[&str]) -> String {
    let regex = Regex::new(r#"\$(\d+)"#).unwrap();
    regex
        .replace_all(template, |captures: &Captures| {
            values.get(index(captures)).unwrap_or(&"")
        })
        .to_string()
}

fn index(captures: &Captures) -> usize {
    captures.get(1).unwrap().as_str().parse().unwrap()
}

// Helper functions
fn wri(mut file: &File, text: &str) {
    file.write(text.as_bytes()).unwrap();
}

fn file_name_from_unit(unit: &Unit) -> String {
    let mut file_name = String::new();
    file_name.push_str(&unit.title);
    file_name.push(' ');
    file_name.push_str(&unit.name);
    file_name.push_str(".json");

    file_name = file_name.replace(" ", "_");

    file_name
}

//Generate JSON
fn gen_header(unit: &Unit) -> String {
    let mut res = template_replace(HEADER1, &[&unit.title, &unit.name]);

    if unit.u_type == UnitType::Adventurer {
        res.push_str(ATTACKTYPE);
    }

    let header2 = template_replace(
        HEADER2,
        &[&unit.stars.to_string(), &unit.limited.to_string()],
    );
    res.push_str(&header2);

    res
}

fn stat_list_borrows(stat: &Vec<String>) -> Vec<&str> {
    let mut stat_str: Vec<&str> = Vec::new();
    for i in 0..6 {
        stat_str.push(&(stat[i]));
    }

    stat_str
}

/*fn borrow_list(ls: &Vec<String>) -> Vec<&str> {
    let mut res: Vec<&str> =
    for el in ls.iter() {

    }
    vec!("hi")
}*/

fn gen_stats(unit: &Unit) -> String {
    let hp = stat_list_borrows(&unit.hp);
    let mp = stat_list_borrows(&unit.mp);
    let stg = stat_list_borrows(&unit.strength);
    let end = stat_list_borrows(&unit.endurance);
    let dex = stat_list_borrows(&unit.dexterity);
    let agi = stat_list_borrows(&unit.agility);
    let mag = stat_list_borrows(&unit.magic);

    let mut res = template_replace(STATS_HP, &hp);
    res.push_str(&template_replace(STATS_MP, &mp));
    res.push_str(&template_replace(STATS_PAT, &stg));
    res.push_str(&template_replace(STATS_MAT, &mag));
    res.push_str(&template_replace(STATS_DEF, &end));
    res.push_str(&template_replace(STATS_STR, &stg));
    res.push_str(&template_replace(STATS_END, &end));
    res.push_str(&template_replace(STATS_DEX, &dex));
    res.push_str(&template_replace(STATS_AGI, &agi));
    res.push_str(&template_replace(STATS_MAG, &mag));

    res
}

/*fn gen_ass_effect(ef: &Effect) -> String {

}*/

fn gen_ass_skill(ass: &Assist) -> String {
    let mut res = template_replace(ASS_SKILLS_HEAD, &[&ass.skill.name]);
    for ef in ass.skill.base_effects.iter() {
        res.push_str(&template_replace(
            ASS_EFFECT,
            &[
                &ef.target.to_json(),
                &ef.attribute.to_json(),
                &ef.modifier.to_string(),
            ],
        ));
        if ef != ass.skill.base_effects.last().unwrap() {
            res.push_str(",")
        }
    }

    res.push_str(ASS_FOOT_SKILL_ONE);
    res.push_str(&template_replace(ASS_SKILL_TWO_HEAD, &[&ass.skill.name]));

    for ef in ass.skill.mlb_effects.iter() {
        res.push_str(&template_replace(
            ASS_EFFECT,
            &[
                &ef.target.to_json(),
                &ef.attribute.to_json(),
                &ef.mod_to_str(),
            ],
        ));
        if ef != ass.skill.mlb_effects.last().unwrap() {
            res.push_str(",")
        }
    }

    res.push_str(ASS_FOOT);

    res
}

/*fn gen_adv_skills() {

}

fn gen_dev_skills() {

}*/

// JSON Generation
fn gen_ass(unit: Unit) {
    let ass = build_ass(unit);

    let file_name = file_name_from_unit(&ass.unit);
    let file = File::create(file_name).unwrap();

    let header = gen_header(&ass.unit);
    wri(&file, &header);

    let stats = gen_stats(&ass.unit);
    wri(&file, &stats);

    let ass_skills = gen_ass_skill(&ass);
    wri(&file, &ass_skills);
}

fn gen_adv(unit: Unit) {
    let adv = build_adv(unit);

    let file_name = file_name_from_unit(&adv.unit);
    let file = File::create(file_name).unwrap();

    println!("Which element is the adventurer? (1: Light, 2: Dark, 3: Fire, 4: Water, 5: Earth, 6: Wind, 7: Thunder");
    let input = read_num();
    let el = match input {
        1 => Element::Light,
        2 => Element::Dark,
        3 => Element::Fire,
        4 => Element::Water,
        5 => Element::Earth,
        6 => Element::Wind,
        7 => Element::Thunder,
        _ => panic!("Please enter a number from 1 to 7"),
    };

    let header = gen_header(&adv.unit);
    let a_type = adv.a_type.to_json().to_string();
    let final_header = template_replace(&header, &[&a_type]);
    wri(&file, &final_header);

    let stats = gen_stats(&adv.unit);
    wri(&file, &stats);

    let at_type_str = match &adv.a_type {
        AttackType::Physical => "physical_attack".to_string(),
        AttackType::Magic => "magic_attack".to_string(),
        AttackType::Balance => "physical_attack/magic_attack".to_string(),
    };
    let el_str = el.to_json().to_string();

    let sa_text = template_replace(ADVSA, &[&el_str, &at_type_str]);
    let skill_text = template_replace(ADVSKILL, &[&el_str, &at_type_str]);
    wri(&file, &sa_text);

    wri(&file, &skill_text);
    wri(&file, "\n            },");
    wri(&file, &skill_text);
    wri(&file, "\n            },");
    wri(&file, &skill_text);

    wri(&file, DEVSKILLS);
}
