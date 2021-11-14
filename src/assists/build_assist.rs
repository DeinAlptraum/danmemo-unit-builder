use super::get_ass_attributes::*;
use crate::get_attributes::*;
use crate::{Assist, AssistEffect, AssistSkill, Unit};

fn build_ass_effect() -> AssistEffect {
    let mut effect = AssistEffect::new();
    effect.attribute = get_ass_attribute();
    effect.target = get_ass_target(&effect.attribute);
    effect.modifier = get_ass_modifier(&effect.attribute);
    effect
}

fn build_ass_skill() -> AssistSkill {
    let mut skill = AssistSkill::new();
    skill.name = get_ass_skill_name();

    let mut base_effects: Vec<AssistEffect> = Vec::new();
    println!("\nLet's build the assist's base skill");
    loop {
        let effect = build_ass_effect();
        base_effects.push(effect);

        println!("\nThe skill currently has the following effects:");
        for ef in base_effects.iter() {
            println!("{}", ef.to_str());
        }

        println!("Does the skill have another effect? n/no: no, anything else: yes");
        let ans = read_str();
        if ans == "n" || ans == "no" {
            break;
        }
    }

    let mut mlb_effects: Vec<AssistEffect> = Vec::new();
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

    skill.base_effects = base_effects;
    skill.mlb_effects = mlb_effects;
    skill
}

pub fn build_ass(unit: Unit) -> Assist {
    let mut ass = Assist::new(unit);
    ass.skill = build_ass_skill();
    ass
}
