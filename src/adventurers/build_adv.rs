use super::adventurer_effects::*;
use super::get_adv_attributes::*;
use crate::adventurer_effects::Buff;
use crate::enums::HumanReadable;
use crate::enums::{Attribute, Speed};
use crate::get_attributes::read_multinum;
use crate::read_str;
use crate::{Adventurer, AdventurerSkill, Unit};

fn build_per_effect_boost() -> PerEffectBuff {
    let mut ef = PerEffectBuff::new();
    ef.attribute = get_per_effect_boost_attr();
    ef.source = get_per_effect_boost_source();
    ef.modifier = get_per_effect_boost_mod();
    ef
}

fn build_per_effect_boosts() -> Vec<PerEffectBuff> {
    let mut efs = Vec::new();
    println!(
        "\nDoes the skill have any per-effect damage buffs? (e.g. \"+40% per each [Self] Str. Buff Skill\"
        y/yes: yes, anything else: no"
    );
    let mut ans = read_str();
    while ans == "y" || ans == "yes" {
        let effect = build_per_effect_boost();
        efs.push(effect);

        println!("\nThe skill currently has the following per-effect damage buffs:");
        for ef in efs.iter() {
            println!("{}", ef.to_str());
        }

        println!(
            "Does the skill have another per-effect damage buff? y/yes: yes, anything else: no"
        );
        ans = read_str();
    }

    efs
}

fn build_dmg_effect() -> Option<Damaging> {
    println!("Which of the following effects does the attack have? (enter applicable separated by spaces, e.g. '2 4'");
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
        dmg_ef.rate_buff = Some(get_rate_buff());
    }
    if ans.contains(&3) {
        dmg_ef.per_effect_buffs = build_per_effect_boosts();
    }
    if ans.contains(&4) {
        dmg_ef.lifesteal = Some(get_lifesteal());
    }
    Some(dmg_ef)
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

        println!("Does the skill have another effect? n/no: no, anything else: yes");
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

fn build_speedless_skill() -> AdventurerSkill {
    let mut sk = AdventurerSkill::new();

    println!("Which of the following effects does the skill have? (enter applicable separated by spaces, e.g. '2 4 5'");
    println!("1: Damaging effect (e.g '[Foe] Hi Fire P.Attack'");
    println!("2: Buffs or Debuffs, including HP Regen skills (e.g. '[Self] +80% Str. /3 turns')");
    println!("3: Buff or Debuff Removal (e.g. '[Foes] removes Str. Buffs exc. Assist Skills')");
    println!("4: Buff or Debuff turn effect (eg. '[Self] M.Resist Debuff -2 turns')");
    println!("5: Nulls, for attacks or ailments");
    println!("6: HP/MP Healing skills (HP regen, HP heal or MP heal");
    println!("7: Ailments (e.g. '[Foes] 35% Sleep')");
    println!("8: Ailment cure");
    let ans = read_multinum();

    sk.name = get_adv_skill_name();
    sk.speed = Speed::None;
    if ans.contains(&1) {
        sk.dmg_effect = build_dmg_effect();
    }
    if ans.contains(&2) {
        sk.buffs = build_effects("Buff/Debuff", build_buff);
    }
    if ans.contains(&3) {
        sk.buff_removals = build_effects("Buff/Debuff Removal", build_buff_removal);
    }
    if ans.contains(&4) {
        sk.buff_turns = build_effects("Buff/Debuff turns affecting", build_buff_turns);
    }
    if ans.contains(&5) {
        sk.nulls = build_effects("Null", build_null);
    }
    if ans.contains(&6) {
        sk = build_heals(sk);
    }
    if ans.contains(&7) {
        sk.ailments = build_effects("Ailment", build_ailment);
    }
    if ans.contains(&8) {
        sk.ailment_cure = true;
    }
    sk
}

fn build_sa() -> AdventurerSkill {
    println!("Let's build the unit's SA");
    let sk = build_speedless_skill();
    sk
}

fn build_adv_skill() -> AdventurerSkill {
    let mut sk = build_speedless_skill();
    sk.speed = get_skill_speed();
    sk
}

fn build_adv_skills() -> Vec<AdventurerSkill> {
    let mut skills = Vec::new();
    for i in 1..4 {
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
