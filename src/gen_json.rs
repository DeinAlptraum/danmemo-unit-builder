use crate::combat_skills::{
    Ailment, Buff, BuffRemove, BuffTurns, Damaging, Heal, KillResist, Null, PerEffectBuff, RateBuff,
};
use crate::enums::{Attribute, DamageType, Element, HumanReadable, Speed, TempBoost, UnitType};
use crate::{json_strings::*, DevelopmentSkill, InstantSkill};
use crate::{Adventurer, AdventurerSkill, Assist, AssistEffect, Unit};
use regex::{Captures, Regex};

// --- Copied from Stack Overflow
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
// ---

// General JSON generators
pub fn gen_header(unit: &Unit) -> String {
    let mut res = template_replace(HEADER1, &[&unit.title, &unit.name]);

    if unit.unit_type == UnitType::Adventurer {
        res.push_str(ATTACKTYPE);
    }

    let header2 = template_replace(
        HEADER2,
        &[&unit.stars.to_string(), &unit.limited.to_string()],
    );
    res.push_str(&header2);

    res
}

fn borrow_string_list(stat: &Vec<String>) -> Vec<&str> {
    let mut stat_str: Vec<&str> = Vec::new();
    for i in 0..6 {
        stat_str.push(&(stat[i]));
    }

    stat_str
}

pub fn gen_stats(unit: &Unit) -> String {
    let hp = borrow_string_list(&unit.hp);
    let mp = borrow_string_list(&unit.mp);
    let stg = borrow_string_list(&unit.strength);
    let end = borrow_string_list(&unit.endurance);
    let dex = borrow_string_list(&unit.dexterity);
    let agi = borrow_string_list(&unit.agility);
    let mag = borrow_string_list(&unit.magic);

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

// Assist JSON generators
pub fn gen_ass_header(ass: &Assist) -> String {
    gen_header(&ass.unit)
}

fn gen_ass_effects(efs: &Vec<AssistEffect>) -> String {
    let mut res = String::new();
    for ef in efs.iter() {
        let modi = match &ef.attribute {
            /*Attribute::NullPhysical | Attribute::NullMagical | Attribute::NullAilment => {
                format!("x{}", &ef.modifier.to_string())
            }*/
            Attribute::AoEResistance | Attribute::STResistance => mod_to_json(ef.modifier),
            _ => ef.mod_to_str(),
        };

        let mut templ = ASS_EFFECT;
        if &ef.attribute == &Attribute::BuffTurns || &ef.attribute == &Attribute::DebuffTurns {
            templ = ASS_TURN_EFFECT;
        }
        res.push_str(&template_replace(
            templ,
            &[&ef.target.to_json(), &ef.attribute.to_json(), &modi],
        ));
        if ef != efs.last().unwrap() {
            res.push_str(",")
        }
    }

    res
}

fn gen_instant_skill(is: &InstantSkill) -> String {
    let mut instant_str = String::new();
    instant_str.push_str(ASS_INSTANT_SKILLS_HEADER);
    let efs;
    if let (Some(dt), Some(el)) = (&is.damage_type, &is.element) {
        efs = gen_skill_effects(false, &is.effects, &dt, &el);
    } else {
        efs = gen_skill_effects(false, &is.effects, &DamageType::Physical, &Element::None);
    }
    instant_str.push_str(&efs);
    instant_str.push_str(ASS_INSTANT_SKILLS_HEADER_TWO);
    instant_str.push_str(&efs);
    instant_str.push_str(ASS_INSTANT_SKILL_FOOTER);
    instant_str
}

pub fn gen_ass_skill(ass: &Assist) -> String {
    let mut res = template_replace(ASS_SKILLS_HEAD, &[&ass.skill.name]);

    let mut ia_base = String::from("");
    let mut ia_mlb = String::from("");
    let mut instant_str = String::from("");
    if let Some(is) = &ass.instant_skill {
        if let Some(ia) = &ass.skill.instant_effect {
            ia_base = template_replace(
                INSTANT_EFFECT,
                &[
                    &ia.base_duration.to_string(),
                    &ia.max_activations.to_string(),
                ],
            );
            ia_mlb = template_replace(
                INSTANT_EFFECT,
                &[
                    &ia.mlb_duration.to_string(),
                    &ia.max_activations.to_string(),
                ],
            );

            instant_str = gen_instant_skill(is);
        }
    }

    let base_effects = gen_ass_effects(&ass.skill.base_effects);
    res.push_str(&base_effects);
    res.push_str(&ia_base);

    res.push_str(ASS_FOOT_SKILL_ONE);
    res.push_str(&template_replace(ASS_SKILL_TWO_HEAD, &[&ass.skill.name]));

    let mlb_effects = gen_ass_effects(&ass.skill.mlb_effects);
    res.push_str(&mlb_effects);
    res.push_str(&ia_mlb);

    res.push_str(ASS_FOOT_SKILL_TWO);

    res.push_str(&instant_str);

    res.push_str(ASS_FOOT);

    res
}

// Adventurer JSON Generators
pub fn gen_adv_header(adv: &Adventurer) -> String {
    let header = gen_header(&adv.unit);
    let at = adv.adventurer_type.to_json();
    let final_header = template_replace(&header, &[&at]);
    final_header
}

// Skill effect generators
// Damaging effect generators
fn gen_dmg_ef(
    is_sa: bool,
    ef: &Damaging,
    dt: &DamageType,
    el: &Element,
    spd: &Speed,
    ef_count: &mut u32,
) -> String {
    let res = gen_effect(
        ef_count,
        is_sa,
        SADMG,
        REGDMG,
        &[
            el.to_json(),
            ef.dmg_mod.to_json(),
            dt.to_json(),
            ef.target.to_json(),
            spd.to_json(),
        ],
    );
    res
}

fn gen_temp_boost(
    is_sa: bool,
    o_tb: &Option<TempBoost>,
    dt: &DamageType,
    ef_count: &mut u32,
) -> String {
    if let Some(tb) = o_tb {
        let res = gen_effect(ef_count, is_sa, SATMPBOOST, REGTMPBOOST, &[tb.to_json(dt)]);
        res
    } else {
        return String::from("");
    }
}

fn gen_rate_buff(is_sa: bool, o_ra: &Option<RateBuff>, ef_count: &mut u32) -> String {
    if let Some(ra) = o_ra {
        let res = gen_effect(
            ef_count,
            is_sa,
            SARATEBUFF,
            REGRATEBUFF,
            &[ra.modifier.to_json(), ra.attribute.to_json()],
        );
        res
    } else {
        return String::from("");
    }
}

fn gen_lifesteal(is_sa: bool, o_ls: &Option<u32>, ef_count: &mut u32) -> String {
    if let Some(ls) = o_ls {
        let res = gen_effect(
            ef_count,
            is_sa,
            SALIFESTEAL,
            REGLIFESTEAL,
            &[&ls.to_string()],
        );
        res
    } else {
        return String::from("");
    }
}

fn gen_per_ef_buffs(is_sa: bool, o_peb: &Option<PerEffectBuff>, ef_count: &mut u32) -> String {
    if let Some(peb) = o_peb {
        let mut res = String::new();
        for attr in &peb.attributes {
            let single_ef = gen_effect(
                ef_count,
                is_sa,
                SAPEB,
                REGPEB,
                &[&peb.modifier.to_string(), &peb.to_json(attr)],
            );
            res.push_str(&single_ef);
        }

        res
    } else {
        return String::from("");
    }
}

fn gen_damaging(
    is_sa: bool,
    o_ef: &Option<Damaging>,
    dt: &DamageType,
    el: &Element,
    spd: &Speed,
    ef_count: &mut u32,
) -> String {
    if let Some(ef) = o_ef {
        let mut res = String::new();
        if *ef_count > 0 {
            res.push_str(",");
        }
        let dmg_ef = gen_dmg_ef(is_sa, ef, dt, el, spd, ef_count);
        res.push_str(&dmg_ef);

        let tb = gen_temp_boost(is_sa, &ef.temp_boost, dt, ef_count);
        res.push_str(&tb);

        let rb = gen_rate_buff(is_sa, &ef.rate_buff, ef_count);
        res.push_str(&rb);

        let ls = gen_lifesteal(is_sa, &ef.lifesteal, ef_count);
        res.push_str(&ls);

        let peb_ef = gen_per_ef_buffs(is_sa, &ef.per_effect_boost, ef_count);
        res.push_str(&peb_ef);

        *ef_count += 1;
        return res;
    } else {
        return String::from("");
    }
}

// Non-damaging generators
fn mod_to_json(modi: i32) -> String {
    if modi >= 0 {
        format!("+{}", modi)
    } else {
        format!("{}", modi)
    }
}

fn gen_buff(is_sa: bool, buff: &Buff, spd: &Speed, ef_count: &mut u32) -> String {
    let modi = match buff.attribute {
        Attribute::HPRegen => buff.modifier.to_string(),
        _ => mod_to_json(buff.modifier),
    };
    let res = gen_effect(
        ef_count,
        is_sa,
        SABUFF,
        REGBUFF,
        &[
            &buff.duration.to_string(),
            &modi,
            &buff.target.to_json(),
            &buff.attribute.to_json(),
            &spd.to_json(),
        ],
    );
    return res;
}

fn gen_buff_removal(is_sa: bool, br: &BuffRemove, spd: &Speed, ef_count: &mut u32) -> String {
    let res = gen_effect(
        ef_count,
        is_sa,
        SABUFFREMOVE,
        REGBUFFREMOVE,
        &[&br.target.to_json(), &br.attr_to_json(), &spd.to_json()],
    );
    res
}

fn gen_buff_turns(is_sa: bool, bt: &BuffTurns, spd: &Speed, ef_count: &mut u32) -> String {
    let res = gen_effect(
        ef_count,
        is_sa,
        SABUFFTURNS,
        REGBUFFTURNS,
        &[
            &mod_to_json(bt.n_turns),
            &bt.target.to_json(),
            &bt.kind.to_json_long(),
            &spd.to_json(),
        ],
    );
    res
}

fn gen_null(is_sa: bool, null: &Null, spd: &Speed, ef_count: &mut u32) -> String {
    let res = gen_effect(
        ef_count,
        is_sa,
        SANULL,
        REGNULL,
        &[
            &null.mod_to_json(),
            &null.target.to_json(),
            &null.kind.to_json(),
            &spd.to_json(),
        ],
    );
    res
}

fn gen_heal(is_sa: bool, o_heal: &Option<Heal>, spd: &Speed, ef_count: &mut u32) -> String {
    if let Some(heal) = o_heal {
        let res = gen_effect(
            ef_count,
            is_sa,
            SAHEAL,
            REGHEAL,
            &[
                &heal.modifier.to_json(),
                &heal.target.to_json(),
                &heal.heal_type.to_json(),
                &spd.to_json(),
            ],
        );
        return res;
    } else {
        return String::from("");
    }
}

fn gen_ailment(is_sa: bool, ail: &Ailment, spd: &Speed, ef_count: &mut u32) -> String {
    let mut sa_templ = SAAIL;
    let mut reg_templ = REGAIL;
    if ail.chance != 100 {
        sa_templ = SAAILCHANCE;
        reg_templ = REGAILCHANCE;
    }
    let res = gen_effect(
        ef_count,
        is_sa,
        sa_templ,
        reg_templ,
        &[
            &ail.chance.to_string(),
            &ail.target.to_json(),
            &ail.kind.to_json(),
            &spd.to_json(),
        ],
    );
    res
}

fn gen_ailment_cure(is_sa: bool, has_cure: bool, spd: &Speed, ef_count: &mut u32) -> String {
    if has_cure {
        let res = gen_effect(ef_count, is_sa, SAAILCURE, REGAILCURE, &[spd.to_json()]);
        return res;
    } else {
        return String::from("");
    }
}

fn gen_kill_resist(
    is_sa: bool,
    o_kr: &Option<KillResist>,
    spd: &Speed,
    ef_count: &mut u32,
) -> String {
    if let Some(kr) = o_kr {
        let res = gen_effect(
            ef_count,
            is_sa,
            SAKILLRES,
            REGKILLRES,
            &[
                &kr.threshold.to_string(),
                kr.target.to_json(),
                &spd.to_json(),
            ],
        );
        return res;
    } else {
        return String::from("");
    }
}

fn gen_additional_action(is_sa: bool, o_aa: &Option<u32>, ef_count: &mut u32) -> String {
    if let Some(aa) = o_aa {
        let res = gen_effect(ef_count, is_sa, SAAASHORT, REGAASHORT, &[&aa.to_string()]);
        return res;
    } else {
        return String::from("");
    }
}

fn gen_effect(
    ef_count: &mut u32,
    is_sa: bool,
    sa_templ: &str,
    reg_templ: &str,
    args: &[&str],
) -> String {
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let ef;
    if is_sa {
        ef = template_replace(sa_templ, args);
    } else {
        ef = template_replace(reg_templ, args);
    }

    res.push_str(&ef);
    *ef_count += 1;
    return res;
}

// Skill generators
pub fn gen_skill_effects(
    is_sa: bool,
    sk: &AdventurerSkill,
    dt: &DamageType,
    el: &Element,
) -> String {
    let mut res = String::new();
    let mut ef_count = 0;

    let sa_dmg = gen_damaging(is_sa, &sk.dmg_effect, &dt, &el, &sk.speed, &mut ef_count);
    res.push_str(&sa_dmg);

    for buff in &sk.buffs {
        let buff_ef = gen_buff(is_sa, buff, &sk.speed, &mut ef_count);
        res.push_str(&buff_ef);
    }

    for br in &sk.buff_removals {
        let br_ef = gen_buff_removal(is_sa, br, &sk.speed, &mut ef_count);
        res.push_str(&br_ef);
    }

    for bt in &sk.buff_turns {
        let bt_ef = gen_buff_turns(is_sa, bt, &sk.speed, &mut ef_count);
        res.push_str(&bt_ef);
    }

    for null in &sk.nulls {
        let null_ef = gen_null(is_sa, null, &sk.speed, &mut ef_count);
        res.push_str(&null_ef);
    }

    let hph = gen_heal(is_sa, &sk.hp_heal, &sk.speed, &mut ef_count);
    res.push_str(&hph);

    let mph = gen_heal(is_sa, &sk.mp_heal, &sk.speed, &mut ef_count);
    res.push_str(&mph);

    for ail in &sk.ailments {
        let ail_ef = gen_ailment(is_sa, ail, &sk.speed, &mut ef_count);
        res.push_str(&ail_ef);
    }

    let ailcure = gen_ailment_cure(is_sa, sk.ailment_cure, &sk.speed, &mut ef_count);
    res.push_str(&ailcure);

    let kr = gen_kill_resist(is_sa, &sk.kill_resist, &sk.speed, &mut ef_count);
    res.push_str(&kr);

    let aa = gen_additional_action(is_sa, &sk.additional_action, &mut ef_count);
    res.push_str(&aa);

    res
}

pub fn gen_sa(adv: &Adventurer) -> String {
    let mut res = template_replace(SAHEADER, &[&adv.sa.name]);

    let efs = gen_skill_effects(true, &adv.sa, &adv.damage_type, &adv.element);
    res.push_str(&efs);
    res.push_str(SAFOOTER);
    res
}

pub fn gen_adv_skills(adv: &Adventurer) -> String {
    let mut res = COMBATHEADER.to_string();

    for sk in &adv.reg_skills {
        res.push_str(&template_replace(REGHEADER, &[&sk.name]));

        let efs = gen_skill_effects(false, sk, &adv.damage_type, &adv.element);
        res.push_str(&efs);
        res.push_str(REGFOOTER);

        if sk != adv.reg_skills.last().unwrap() {
            res.push_str(",");
        }
    }

    res.push_str(COMBATFOOTER);

    res
}

pub fn gen_aa(adv: &Adventurer) -> String {
    let mut res = AASECTIONHEADER.to_string();

    for aa in &adv.additional_actions {
        res.push_str(&template_replace(AAHEADER, &[&aa.name]));
        let efs = gen_skill_effects(false, &aa, &adv.damage_type, &adv.element);
        res.push_str(&efs);
        res.push_str(AAFOOTER);

        if aa != adv.additional_actions.last().unwrap() {
            res.push_str(",");
        }
    }

    res.push_str(AASECTIONFOOTER);

    res
}

// Development skills
pub fn gen_dev_skill(ds: &DevelopmentSkill) -> String {
    let mut res = template_replace(DSHEADER, &[&ds.to_str()]);

    let descs = ds.effect.get_descriptions();
    let modis = ds.effect.get_modifiers();

    for i in 0..(descs.len()) {
        let modi = match &modis.get(i) {
            Some(s) => s,
            None => "",
        };
        let ef = template_replace(DSEFFECT, &[&descs[i], &modi]);
        res.push_str(&ef);
        if i != descs.len() - 1 {
            res.push_str(",");
        }
    }

    res.push_str(DSFOOTER);
    res
}

pub fn gen_dev_skills(adv: &Adventurer) -> String {
    let mut res = DEVHEADER.to_string();

    for ds in &adv.dev_skills {
        let ef_str = gen_dev_skill(ds);
        res.push_str(&ef_str);
        if ds != adv.dev_skills.last().unwrap() {
            res.push_str(",");
        }
    }

    res.push_str(DEVFOOTER);
    res
}
