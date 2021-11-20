use super::adventurer_effects::*;
use super::get_adv_attributes::*;
use crate::adventurer_effects::Buff;
use crate::enums::HumanReadable;
use crate::enums::{Attribute, Speed};
use crate::get_attributes::read_multinum;
use crate::read_str;
use crate::{Adventurer, AdventurerSkill, Unit};

fn build_rate_buff() -> RateBuff {
    let mut ef = RateBuff::new();
    ef.attribute = get_rate_buff_attr();
    ef.modifier = get_rate_buff_mod();
    ef
}

fn build_per_effect_boost() -> PerEffectBuff {
    let mut ef = PerEffectBuff::new();
    ef.attributes = get_per_effect_boost_attrs();
    ef.source = get_per_effect_boost_source();
    ef.kind = get_per_effect_boost_kind();
    ef.modifier = get_per_effect_boost_mod();

    ef = compute_per_effect_boost_combos(ef);
    ef
}

fn compute_per_effect_boost_combos(ef: PerEffectBuff) -> PerEffectBuff {
    let mut str_present = false;
    let mut mag_present = false;
    for attr in &ef.attributes {
        if attr == &Attribute::Strength || attr == &Attribute::Magic {
            if attr == &Attribute::Strength {
                str_present = true;
            } else {
                mag_present = true;
            }
        }
    }

    let mut combo_ef = PerEffectBuff::new();
    if str_present && mag_present {
        combo_ef.attributes.push(Attribute::StrengthAndMagic);
        combo_ef.source = ef.source;
        combo_ef.kind = ef.kind;
        combo_ef.modifier = ef.modifier;

        for attr in &ef.attributes {
            let cp_attr = attr.clone();
            if cp_attr != Attribute::Strength && cp_attr != Attribute::Magic {
                combo_ef.attributes.push(cp_attr);
            }
        }

        combo_ef
    } else {
        ef
    }
}

fn build_dmg_effect() -> Damaging {
    println!("\nWhich of the following effects does the attack have? (enter applicable separated by spaces, e.g. '2 4'");
    println!("1: Temporary boost (e.g 'temp. Str. Boost'");
    println!("2: Rate buffs (e.g. 'Ultra Unguard Rate')");
    println!("3: Per effect boosts (e.g. '+40% per each [Self] Dex. Buff Skill')");
    println!("4: Life Steal");
    let ans = read_multinum();

    let mut dmg_ef = Damaging::new();
    dmg_ef.target = get_dmg_target();
    dmg_ef.dmg_mod = get_dmg_mod();
    if ans.contains(&1) {
        dmg_ef.temp_boost = Some(get_temp_boost());
    }
    if ans.contains(&2) {
        dmg_ef.rate_buff = Some(build_rate_buff());
    }
    if ans.contains(&3) {
        dmg_ef.per_effect_boost = Some(build_per_effect_boost());
    }
    if ans.contains(&4) {
        dmg_ef.lifesteal = Some(get_lifesteal());
    }
    dmg_ef
}

fn build_buff() -> Buff {
    let mut bf = Buff::new();
    bf.target = get_buff_target();
    bf.attribute = get_buff_attribute();
    bf.modifier = get_buff_mod(&bf.attribute);
    bf.duration = get_duration();
    bf
}

fn build_buff_removal() -> BuffRemove {
    let mut br = BuffRemove::new();
    br.target = get_buff_removal_target();
    br.kind = get_buff_removal_kind();
    br.attribute = get_buff_removal_attribute(&br.kind);
    br
}

fn compute_buff_removal_combos(efs: Vec<BuffRemove>) -> Vec<BuffRemove> {
    let mut str_present = false;
    let mut mag_present = false;
    let mut found_ef = &BuffRemove::new();
    for ef in &efs {
        if ef.attribute == Attribute::Strength || ef.attribute == Attribute::Magic {
            found_ef = ef.clone();
            if ef.attribute == Attribute::Strength {
                str_present = true;
            } else {
                mag_present = true;
            }
        }
    }
    if str_present && mag_present {
        let mut combo_ef = BuffRemove::new();
        combo_ef.attribute = Attribute::StrengthAndMagic;
        combo_ef.target = found_ef.target;
        combo_ef.kind = found_ef.kind;

        let mut new_efs = Vec::new();
        new_efs.push(combo_ef);
        for ef in efs {
            if ef.attribute != Attribute::Strength && ef.attribute != Attribute::Magic {
                new_efs.push(ef);
            }
        }
        new_efs
    } else {
        efs
    }
}

fn build_buff_turns() -> BuffTurns {
    let mut bt = BuffTurns::new();
    bt.target = get_buff_turns_target();
    bt.kind = get_buff_turns_kind();
    bt.n_turns = get_buff_turns_number(&bt.kind);
    bt
}

fn build_null() -> Null {
    let mut null = Null::new();
    null.target = get_null_target();
    null.kind = get_null_kind();
    null.amount = get_null_amount();
    null.chance = get_null_chance();
    null
}

fn build_ailment() -> Ailment {
    let mut ail = Ailment::new();
    ail.target = get_ail_target();
    ail.kind = get_ail_kind();
    ail.chance = get_ail_chance();
    ail
}

fn build_effects<T: HumanReadable>(
    skilltype: &str,
    builder: fn() -> T,
    combo: fn(Vec<T>) -> Vec<T>,
) -> Vec<T> {
    let mut effects: Vec<T> = Vec::new();
    println!("\nLet's build the {} effects", skilltype);
    loop {
        let ef = builder();
        effects.push(ef);

        println!("\nThe skill currently has the following effects:");
        for ef in &effects {
            println!("{}", ef.to_str());
        }

        println!(
            "Does the skill have another {} effect? n/no: no, anything else: yes",
            skilltype
        );
        let ans = read_str();
        if ans == "n" || ans == "no" {
            break;
        }
    }

    let combo_effects = combo(effects);

    combo_effects
}

fn empty_combo<T: HumanReadable>(efs: Vec<T>) -> Vec<T> {
    efs
}

fn build_heals(mut sk: AdventurerSkill) -> AdventurerSkill {
    println!("Which type of healing effects does the skill have? (enter applicable separated by spaces, e.g. '1 2'");
    println!("1: HP healing");
    println!("2: MP healing");
    let ans = read_multinum();

    if ans.contains(&1) {
        let heal = Heal {
            target: get_heal_target(),
            heal_type: Attribute::Heal,
            modifier: get_hp_heal_modifier(),
        };
        sk.hp_heal = Some(heal);
    }
    if ans.contains(&2) {
        let heal = Heal {
            target: get_heal_target(),
            heal_type: Attribute::MPHeal,
            modifier: get_mp_heal_modifier(),
        };
        sk.mp_heal = Some(heal);
    }

    sk
}

fn build_kill_resist() -> KillResist {
    let mut kr = KillResist::new();
    kr.target = get_kill_resist_target();
    kr.threshold = get_kill_resist_threshold();
    kr
}

fn build_additional_action() -> AdditionalAction {
    let mut aa = AdditionalAction::new();
    aa.effect = get_additional_action_effect();
    aa.quantity = get_additional_action_quantity();
    aa
}

fn build_speedless_skill(is_sa: bool) -> AdventurerSkill {
    let mut sk = AdventurerSkill::new();

    sk.name = get_adv_skill_name();

    println!("\nWhich of the following effects does the skill have? (enter applicable separated by spaces, e.g. '2 4 5'");
    println!("1: Damaging effect (e.g '[Foe] Hi Fire P.Attack'");
    println!("2: Buffs or Debuffs, including HP Regen skills (e.g. '[Self] +80% Str. /3 turns')");
    println!("3: Buff or Debuff Removal (e.g. '[Foes] removes Str. Buffs exc. Assist Skills')");
    println!("4: Buff or Debuff turn effect (e.g. '[Self] M.Resist Debuff -2 turns')");
    println!("5: Nulls, for attacks or ailments");
    println!("6: HP/MP Healing skills (HP regen, HP heal or MP heal");
    println!("7: Ailments (e.g. '[Foes] 35% Sleep')");
    println!("8: Ailment cure");
    println!("9: Kill resist (e.g. '[Allies] Avoids K.O x1 only when HP >= 10%)");
    if !is_sa {
        println!("10: Additional actions");
    }
    let ans = read_multinum();

    sk.speed = Speed::None;
    if ans.contains(&1) {
        println!();
        sk.dmg_effect = Some(build_dmg_effect());
    }
    if ans.contains(&2) {
        println!();
        sk.buffs = build_effects("Buff/Debuff", build_buff, empty_combo);
    }
    if ans.contains(&3) {
        println!();
        sk.buff_removals = build_effects(
            "Buff/Debuff Removal",
            build_buff_removal,
            compute_buff_removal_combos,
        );
    }
    if ans.contains(&4) {
        println!();
        sk.buff_turns = build_effects("Buff/Debuff turns affecting", build_buff_turns, empty_combo);
    }
    if ans.contains(&5) {
        println!();
        sk.nulls = build_effects("Null", build_null, empty_combo);
    }
    if ans.contains(&6) {
        println!();
        sk = build_heals(sk);
    }
    if ans.contains(&7) {
        println!();
        sk.ailments = build_effects("Ailment", build_ailment, empty_combo);
    }
    if ans.contains(&8) {
        sk.ailment_cure = true;
    }
    if ans.contains(&9) {
        sk.kill_resist = Some(build_kill_resist());
    }
    if ans.contains(&10) && !is_sa {
        println!();
        sk.additional_action = Some(build_additional_action());
    }

    sk
}

fn build_sa() -> AdventurerSkill {
    print!("\x1B[2J\x1B[1;1H"); // clears console
    println!("Let's build the unit's SA");
    let sk = build_speedless_skill(true);
    sk
}

fn build_adv_skill() -> AdventurerSkill {
    let mut sk = build_speedless_skill(false);
    sk.speed = get_skill_speed();
    sk
}

fn build_adv_skills() -> Vec<AdventurerSkill> {
    let mut skills = Vec::new();
    for i in 1..4 {
        print!("\x1B[2J\x1B[1;1H"); // clears console
        println!("Let's build the {}. regular skill", i);
        skills.push(build_adv_skill());
    }
    skills
}

pub fn build_adv(unit: Unit) -> Adventurer {
    let mut adv = Adventurer::new(unit);
    adv.adventurer_type = get_adventurer_type();
    adv.damage_type = get_dmg_type(&adv.adventurer_type);
    adv.element = get_element();
    adv.sa = build_sa();
    adv.reg_skills = build_adv_skills();
    adv
}
