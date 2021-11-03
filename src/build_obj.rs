use crate::adventurer::Adventurer;
use crate::ass_skill::AssistSkill;
use crate::assist::Assist;
use crate::effect::Effect;
use crate::get_attributes::*;
use crate::unit::Unit;

pub fn build_unit() -> Unit {
    let unit_type = get_unit_type();
    let title = get_title();
    let name = get_name();
    let stars = get_stars();
    let limited = get_limited();
    let hp = get_six_stats("HP");
    let mp = get_six_stats("MP");
    let stg = get_six_stats("strength");
    let end = get_six_stats("endurance");
    let dex = get_six_stats("dexterity");
    let agi = get_six_stats("agility");
    let mag = get_six_stats("magic");

    let mut unit = Unit::new();
    unit.u_type = unit_type;
    unit.title = title;
    unit.name = name;
    unit.stars = stars;
    unit.limited = limited;
    unit.hp = hp;
    unit.mp = mp;
    unit.strength = stg;
    unit.endurance = end;
    unit.dexterity = dex;
    unit.agility = agi;
    unit.magic = mag;

    unit
}

fn build_ass_effect() -> Effect {
    let attr = get_ass_attribute();
    let target = get_ass_target(&attr);
    let modi = get_ass_modifier(&attr);

    let mut effect = Effect::new();
    effect.attribute = attr;
    effect.target = target;
    effect.modifier = modi;
    effect
}

fn build_ass_skill() -> AssistSkill {
    let skill_name = get_ass_skill_name();

    let mut base_effects: Vec<Effect> = Vec::new();
    println!("\nLet's build the assist's base skill");
    loop {
        let effect = build_ass_effect();
        base_effects.push(effect);

        println!("\nThe skill currently has the following effects:");
        for ef in base_effects.iter() {
            println!("{}", ef.to_str());
        }
        println!("Does the skill have another effect? (y: yes, anything else: no)");
        if read_str() != "y" {
            break;
        }
    }

    let mut mlb_effects: Vec<Effect> = Vec::new();
    for ef in base_effects.iter() {
        println!(
            "\nAt base, the effect was: {}\nWhat is the modifer at mlb?",
            ef.to_str()
        );
        let modi = get_ass_modifier(&ef.attribute);
        let mut mlb_ef = ef.clone();
        mlb_ef.modifier = modi;
        mlb_effects.push(mlb_ef);
    }

    let mut skill = AssistSkill::new();
    skill.name = skill_name;
    skill.base_effects = base_effects;
    skill.mlb_effects = mlb_effects;
    skill
}

pub fn build_ass(unit: Unit) -> Assist {
    let sk = build_ass_skill();

    let mut ass = Assist::new(unit);
    ass.skill = sk;
    ass
}

pub fn build_adv(unit: Unit) -> Adventurer {
    let a_type = get_attack_type();

    let mut adv = Adventurer::new(unit);
    adv.a_type = a_type;
    adv
}
