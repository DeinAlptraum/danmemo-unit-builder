use super::get_ass_attributes::*;
use crate::build_nameless_skill;
use crate::{get_attributes::*, InstantEffect, InstantSkill};
use crate::{Assist, AssistEffect, AssistSkill, Unit};

fn build_ass_effect() -> AssistEffect {
    let mut effect = AssistEffect::new();
    effect.attribute = get_ass_attribute();
    effect.target = get_ass_target(&effect.attribute);
    effect.modifier = get_ass_modifier(&effect.attribute);
    effect
}

fn build_base_effects() -> Vec<AssistEffect> {
    let mut efs: Vec<AssistEffect> = Vec::new();
    println!("\nLet's build the assist's base skill");
    loop {
        let effect = build_ass_effect();
        efs.push(effect);

        println!("\nThe skill currently has the following effects:");
        for ef in &efs {
            println!("{}", ef.to_str());
        }

        println!("Does the skill have another regular effect? n/no: no, anything else: yes");
        let ans = read_str();
        if ans == "n" || ans == "no" {
            break;
        }
    }

    efs
}

fn build_mlb_effects(base_efs: &Vec<AssistEffect>) -> Vec<AssistEffect> {
    let mut mlb_effects: Vec<AssistEffect> = Vec::new();
    for ef in base_efs {
        println!(
            "\nAt base, the effect was: {}\nWhat is the modifer at MLB?",
            ef.to_str()
        );
        let modi = get_ass_modifier(&ef.attribute);
        let mut mlb_ef = ef.clone();
        mlb_ef.modifier = modi;
        mlb_effects.push(mlb_ef);
    }

    mlb_effects
}

fn build_instant_effect() -> Option<InstantEffect> {
    println!("\nDoes the skill have an instant effect? n/no: no, anything else: yes");
    let ans = read_str();
    if ans == "n" || ans == "no" {
        return None;
    } else {
        let mut ia = InstantEffect::new();
        ia.base_duration = get_ia_base_duration();
        ia.mlb_duration = get_ia_mlb_duration();
        ia.max_activations = get_ia_max_activations();
        return Some(ia);
    }
}

fn build_ass_skill() -> AssistSkill {
    let mut skill = AssistSkill::new();
    skill.name = get_ass_skill_name();

    skill.instant_effect = build_instant_effect();

    skill.base_effects = build_base_effects();
    skill.mlb_effects = build_mlb_effects(&skill.base_effects);
    skill
}

fn build_instant_skill() -> InstantSkill {
    println!("\n\nLets build the instant effects");
    let mut is = InstantSkill::new();
    is.effects = build_nameless_skill(true, &mut false);
    if let Some(_) = is.effects.dmg_effect {
        is.damage_type = Some(get_instant_damage_type());
        is.element = Some(get_instant_element());
    }
    is
}

pub fn build_ass(unit: Unit) -> Assist {
    let mut ass = Assist::new(unit);
    ass.skill = build_ass_skill();
    if let Some(_) = &ass.skill.instant_effect {
        ass.instant_skill = Some(build_instant_skill());
    }
    ass
}
