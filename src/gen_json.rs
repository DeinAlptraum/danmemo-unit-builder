use crate::adventurer_effects::{
    Ailment, Buff, BuffRemove, BuffTurns, Damaging, Heal, Null, PerEffectBuff,
};
use crate::enums::{DamageType, Element, RateAttribute, Speed, TempBoost, UnitType};
use crate::{Adventurer, AdventurerSkill, json_strings::*};
use crate::{Assist, AssistEffect, Unit};
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
        res.push_str(&template_replace(
            ASS_EFFECT,
            &[
                &ef.target.to_json(),
                &ef.attribute.to_json(),
                &ef.modifier.to_string(),
            ],
        ));
        if ef != efs.last().unwrap() {
            res.push_str(",")
        }
    }

    res
}

pub fn gen_ass_skill(ass: &Assist) -> String {
    let mut res = template_replace(ASS_SKILLS_HEAD, &[&ass.skill.name]);

    let base_effects = gen_ass_effects(&ass.skill.base_effects);
    res.push_str(&base_effects);

    res.push_str(ASS_FOOT_SKILL_ONE);
    res.push_str(&template_replace(ASS_SKILL_TWO_HEAD, &[&ass.skill.name]));

    let mlb_effects = gen_ass_effects(&ass.skill.mlb_effects);
    res.push_str(&mlb_effects);

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
fn gen_dmg_ef(is_sa: bool, ef: &Damaging, dt: &DamageType, el: &Element, spd: &Speed) -> String {
    let mut res = String::new();
    if is_sa {
        let sa_dmg = template_replace(
            SADMG,
            &[
                el.to_json(),
                ef.dmg_mod.to_json(),
                dt.to_json(),
                ef.target.to_json(),
                spd.to_json(),
            ],
        );
        res.push_str(&sa_dmg);
    } else {
        let reg_dmg = template_replace(
            REGDMG,
            &[
                el.to_json(),
                ef.dmg_mod.to_json(),
                dt.to_json(),
                ef.target.to_json(),
                spd.to_json(),
            ],
        );
        res.push_str(&reg_dmg);
    }
    res
}

fn gen_temp_boost(
    is_sa: bool,
    o_tb: &Option<TempBoost>,
    dt: &DamageType,
    ef_count: &mut u32,
) -> String {
    if let Some(tb) = o_tb {
        let mut res = String::new();
        if *ef_count > 0 {
            res.push_str(",");
        }

        let tb_ef;
        if is_sa {
            tb_ef = template_replace(SATMPBOOST, &[tb.to_json(dt)]);
        } else {
            tb_ef = template_replace(REGTMPBOOST, &[tb.to_json(dt)]);
        }

        res.push_str(&tb_ef);
        *ef_count += 1;
        return res;
    } else {
        return String::from("");
    }
}

fn gen_rate_buff(is_sa: bool, o_ra: &Option<RateAttribute>, ef_count: &mut u32) -> String {
    if let Some(ra) = o_ra {
        let mut res = String::new();
        if *ef_count > 0 {
            res.push_str(",");
        }

        let rb_ef;
        if is_sa {
            rb_ef = template_replace(SARATEBUFF, &[ra.to_json()]);
        } else {
            rb_ef = template_replace(REGRATEBUFF, &[ra.to_json()]);
        }

        res.push_str(&rb_ef);
        *ef_count += 1;
        return res;
    } else {
        return String::from("");
    }
}

fn gen_lifesteal(is_sa: bool, o_ls: &Option<u32>, ef_count: &mut u32) -> String {
    if let Some(ls) = o_ls {
        let mut res = String::new();
        if *ef_count > 0 {
            res.push_str(",");
        }

        let ls_ef;
        if is_sa {
            ls_ef = template_replace(SALIFESTEAL, &[&ls.to_string()]);
        } else {
            ls_ef = template_replace(REGLIFESTEAL, &[&ls.to_string()]);
        }

        res.push_str(&ls_ef);
        *ef_count += 1;
        return res;
    } else {
        return String::from("");
    }
}

fn gen_per_ef_buff(is_sa: bool, peb: &PerEffectBuff, ef_count: &mut u32) -> String {
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let peb_ef;
    if is_sa {
        peb_ef = template_replace(SAPEB, &[&peb.to_json(), &peb.modifier.to_string()]);
    } else {
        peb_ef = template_replace(REGPEB, &[&peb.to_json(), &peb.modifier.to_string()]);
    }

    res.push_str(&peb_ef);
    *ef_count += 1;
    return res;
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
        let dmg_ef = gen_dmg_ef(is_sa, ef, dt, el, spd);
        res.push_str(&dmg_ef);

        let tb = gen_temp_boost(is_sa, &ef.temp_boost, dt, ef_count);
        res.push_str(&tb);

        let rb = gen_rate_buff(is_sa, &ef.rate_buff, ef_count);
        res.push_str(&rb);

        let ls = gen_lifesteal(is_sa, &ef.lifesteal, ef_count);
        res.push_str(&ls);

        for peb in &ef.per_effect_buffs {
            let peb_ef = gen_per_ef_buff(is_sa, &peb, ef_count);
            res.push_str(&peb_ef);
        }

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
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let buff_ef;
    if is_sa {
        buff_ef = template_replace(
            SABUFF,
            &[
                &buff.duration.to_string(),
                &mod_to_json(buff.modifier),
                &buff.target.to_json(),
                &buff.attribute.to_json(),
                &spd.to_json(),
            ],
        );
    } else {
        buff_ef = template_replace(
            REGBUFF,
            &[
                &buff.duration.to_string(),
                &mod_to_json(buff.modifier),
                &buff.target.to_json(),
                &buff.attribute.to_json(),
                &spd.to_json(),
            ],
        );
    }

    res.push_str(&buff_ef);
    *ef_count += 1;
    return res;
}

fn gen_buff_removal(is_sa: bool, br: &BuffRemove, spd: &Speed, ef_count: &mut u32) -> String {
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let br_ef;
    if is_sa {
        br_ef = template_replace(
            SABUFFREMOVE,
            &[&br.target.to_json(), &br.attr_to_json(), &spd.to_json()],
        );
    } else {
        br_ef = template_replace(
            REGBUFFREMOVE,
            &[&br.target.to_json(), &br.attr_to_json(), &spd.to_json()],
        );
    }

    res.push_str(&br_ef);
    *ef_count += 1;
    return res;
}

fn gen_buff_turns(is_sa: bool, bt: &BuffTurns, spd: &Speed, ef_count: &mut u32) -> String {
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let bt_ef;
    if is_sa {
        bt_ef = template_replace(
            SABUFFTURNS,
            &[
                &mod_to_json(bt.n_turns),
                &bt.target.to_json(),
                &bt.kind.to_json_long(),
                &spd.to_json(),
            ],
        );
    } else {
        bt_ef = template_replace(
            REGBUFFTURNS,
            &[
                &mod_to_json(bt.n_turns),
                &bt.target.to_json(),
                &bt.kind.to_json_long(),
                &spd.to_json(),
            ],
        );
    }

    res.push_str(&bt_ef);
    *ef_count += 1;
    return res;
}

fn gen_null(is_sa: bool, null: &Null, spd: &Speed, ef_count: &mut u32) -> String {
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let null_ef;
    if is_sa {
        null_ef = template_replace(
            SANULL,
            &[
                &null.mod_to_json(),
                &null.target.to_json(),
                &null.kind.to_json(),
                &spd.to_json(),
            ],
        );
    } else {
        null_ef = template_replace(
            REGNULL,
            &[
                &null.mod_to_json(),
                &null.target.to_json(),
                &null.kind.to_json(),
                &spd.to_json(),
            ],
        );
    }

    res.push_str(&null_ef);
    *ef_count += 1;
    return res;
}

fn gen_heal(is_sa: bool, o_heal: &Option<Heal>, spd: &Speed, ef_count: &mut u32) -> String {
    if let Some(heal) = o_heal {
        let mut res = String::new();
        if *ef_count > 0 {
            res.push_str(",");
        }

        let heal_ef;
        if is_sa {
            heal_ef = template_replace(
                SAHEAL,
                &[
                    &heal.modifier.to_json(),
                    &heal.target.to_json(),
                    &heal.heal_type.to_json(),
                    &spd.to_json(),
                ],
            );
        } else {
            heal_ef = template_replace(
                REGHEAL,
                &[
                    &heal.modifier.to_json(),
                    &heal.target.to_json(),
                    &heal.heal_type.to_json(),
                    &spd.to_json(),
                ],
            );
        }

        res.push_str(&heal_ef);
        *ef_count += 1;
        return res;
    } else {
        return String::from("");
    }
}

fn gen_ailment(is_sa: bool, ail: &Ailment, spd: &Speed, ef_count: &mut u32) -> String {
    let mut res = String::new();
    if *ef_count > 0 {
        res.push_str(",");
    }

    let ail_ef;
    if is_sa {
        ail_ef = template_replace(
            SAAIL,
            &[
                &ail.chance.to_string(),
                &ail.target.to_json(),
                &ail.kind.to_json(),
                &spd.to_json(),
            ],
        );
    } else {
        ail_ef = template_replace(
            REGAIL,
            &[
                &ail.chance.to_string(),
                &ail.target.to_json(),
                &ail.kind.to_json(),
                &spd.to_json(),
            ],
        );
    }

    res.push_str(&ail_ef);
    *ef_count += 1;
    return res;
}

fn gen_ailment_cure(is_sa: bool, has_cure: bool, spd: &Speed, ef_count: &mut u32) -> String {
    if has_cure {
        let mut res = String::new();
        if *ef_count > 0 {
            res.push_str(",");
        }

        let ail_cure_ef;
        if is_sa {
            ail_cure_ef = template_replace(SAAILCURE, &[&spd.to_json()]);
        } else {
            ail_cure_ef = template_replace(REGAILCURE, &[&spd.to_json()]);
        }

        res.push_str(&ail_cure_ef);
        *ef_count += 1;
        return res;
    } else {
        return String::from("");
    }
}

// Skill generators
pub fn gen_skill_effects(is_sa: bool, sk: &AdventurerSkill, dt: &DamageType, el: &Element) -> String {
    let mut res = String::new();
    let mut ef_count = 0;

    let sa_dmg = gen_damaging(
        is_sa,
        &sk.dmg_effect,
        &dt,
        &el,
        &Speed::None,
        &mut ef_count,
    );
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

pub fn gen_dev_skills(adv: &Adventurer) -> String {
    DEVSKILLS.to_string()
}
