use super::combat_skills::*;
use super::get_adv_attributes::*;
use crate::enums::{Attribute, HumanReadable, Speed};
use crate::get_attributes::{read_multinum, read_str};
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
    ef
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

fn build_effects<T: HumanReadable>(skilltype: &str, builder: fn() -> T) -> Vec<T> {
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

    effects
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

fn build_nameless_skill(is_sa: bool, has_aa: &mut bool) -> AdventurerSkill {
    let mut sk = AdventurerSkill::new();

    println!("\nWhich of the following effects does the skill have? (enter applicable separated by spaces, e.g. '2 4 5'");
    println!("1: Damaging effect (e.g '[Foe] Hi Fire P.Attack'");
    println!("2: Buffs or Debuffs, including HP Regen skills (e.g. '[Self] +80% Str. /3 turns')");
    println!("3: Buff or Debuff Removal (e.g. '[Foes] removes Str. Buffs exc. Assist Skills')");
    println!("4: Buff or Debuff turn effect (e.g. '[Self] Status Debuff-2 turns')");
    println!("5: Nulls, for attacks or ailments");
    println!("6: HP/MP Healing skills (HP regen, HP heal or MP heal");
    println!("7: Ailments (e.g. '[Foes] 35% Sleep')");
    println!("8: Ailment cure");
    println!("9: Kill resist (e.g. '[Allies] Avoids K.O x1 only when HP >= 10%)");
    if !is_sa && !*has_aa {
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
        sk.buffs = build_effects("Buff/Debuff", build_buff);
    }
    if ans.contains(&3) {
        println!();
        sk.buff_removals = build_effects("Buff/Debuff Removal", build_buff_removal);
    }
    if ans.contains(&4) {
        println!();
        sk.buff_turns = build_effects("Buff/Debuff turns affecting", build_buff_turns);
    }
    if ans.contains(&5) {
        println!();
        sk.nulls = build_effects("Null", build_null);
    }
    if ans.contains(&6) {
        println!();
        sk = build_heals(sk);
    }
    if ans.contains(&7) {
        println!();
        sk.ailments = build_effects("Ailment", build_ailment);
    }
    if ans.contains(&8) {
        sk.ailment_cure = true;
    }
    if ans.contains(&9) {
        sk.kill_resist = Some(build_kill_resist());
    }
    if ans.contains(&10) && !is_sa {
        println!();
        *has_aa = true;
        sk.additional_action = Some(get_additional_action_quantity());
    }

    sk
}

fn build_speedless_skill(is_sa: bool, has_aa: &mut bool) -> AdventurerSkill {
    let name = get_adv_skill_name();
    let mut sk = build_nameless_skill(is_sa, has_aa);
    sk.name = name;

    sk
}

fn build_sa() -> AdventurerSkill {
    print!("\x1B[2J\x1B[1;1H"); // clears console
    println!("Let's build the unit's SA");
    let sk = build_speedless_skill(true, &mut false);
    sk
}

fn build_adv_skill(has_aa: &mut bool) -> AdventurerSkill {
    let mut sk = build_speedless_skill(false, has_aa);
    sk.speed = get_skill_speed();
    sk
}

fn build_adv_skills(has_aa: &mut bool) -> Vec<AdventurerSkill> {
    let mut skills = Vec::new();
    for i in 1..4 {
        print!("\x1B[2J\x1B[1;1H"); // clears console
        println!("Let's build the {}. regular skill", i);
        skills.push(build_adv_skill(has_aa));
    }
    skills
}

fn build_additional_action() -> AdventurerSkill {
    print!("\x1B[2J\x1B[1;1H"); // clears console
    println!("Let's build the unit's additional action");
    let sk = build_nameless_skill(true, &mut false);
    sk
}

pub fn build_adv(unit: Unit) -> Adventurer {
    let mut adv = Adventurer::new(unit);
    adv.adventurer_type = get_adventurer_type();
    adv.damage_type = get_dmg_type(&adv.adventurer_type);
    adv.element = get_element();
    adv.sa = build_sa();

    let mut has_aa = false;
    adv.reg_skills = build_adv_skills(&mut has_aa);
    if has_aa {
        adv.additional_action = Some(build_additional_action());
    }
    adv
}
