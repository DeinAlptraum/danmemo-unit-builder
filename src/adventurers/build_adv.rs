use std::collections::HashSet;

use super::combat_skills::*;
use super::get_adv_attributes::*;
use crate::enums::{
    Attribute, BuffType, DevelopmentSkillType, HumanReadable, SkillRank, Speed, Target,
};
use crate::get_attributes::{read_multinum, read_nonempty_str, read_str};
use crate::DevelopmentSkill;
use crate::{Adventurer, AdventurerSkill, Unit};

// Damaging
fn build_rate_buff() -> RateBuff {
    let mut ef = RateBuff::new();
    ef.attribute = get_rate_buff_attr();
    ef.modifier = get_rate_buff_mod();
    ef
}

fn build_per_effect_boost() -> PerEffectBuff {
    let mut ef = PerEffectBuff::new();
    ef.attributes = get_per_effect_boost_attrs();
    let ails: HashSet<Attribute> = vec![
        Attribute::Sleep,
        Attribute::Stun,
        Attribute::Seal,
        Attribute::Slow,
        Attribute::Taunt,
        Attribute::Poison,
        Attribute::Charm,
    ]
    .into_iter()
    .collect();
    if ails
        .intersection(&ef.attributes.clone().into_iter().collect())
        .count()
        == 0
    {
        ef.source = get_per_effect_boost_source();
        ef.kind = get_per_effect_boost_kind();
    } else {
        ef.source = Target::Foe;
        ef.kind = BuffType::Debuff;
    }
    ef.modifier = get_per_effect_boost_mod();
    ef
}

fn build_dmg_effect() -> Damaging {
    println!("\nWhich of the following effects does the attack have? (enter applicable separated by spaces, e.g. '2 4')");
    println!("1: Temporary boost (e.g 'temp. Str. Boost')");
    println!(
        "2: Per effect/ailment on target boosts (e.g. '+40% per each [Self] Dex. Buff Skill')"
    );
    println!("3: Rate buffs (e.g. 'Ultra Unguard Rate')");
    println!("4: Life Steal");
    let ans = read_multinum();

    let mut dmg_ef = Damaging::new();
    dmg_ef.target = get_dmg_target();
    dmg_ef.dmg_mod = get_dmg_mod();
    if ans.contains(&1) {
        dmg_ef.temp_boost = Some(get_temp_boost());
    }
    if ans.contains(&2) {
        dmg_ef.per_effect_boost = Some(build_per_effect_boost());
    }
    if ans.contains(&3) {
        dmg_ef.rate_buff = Some(build_rate_buff());
    }
    if ans.contains(&4) {
        dmg_ef.lifesteal = Some(get_lifesteal());
    }
    dmg_ef
}

// Non-damaging
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
    ail.chance = get_ail_chance(&ail.kind);
    ail
}

fn build_effects<T: HumanReadable>(skilltype: &str, builder: fn() -> T) -> Vec<T> {
    let mut effects: Vec<T> = Vec::new();
    println!("\nLet's build the {skilltype} effects");
    loop {
        let ef = builder();
        effects.push(ef);

        println!("\nThe skill currently has the following effects:");
        for ef in &effects {
            println!("{}", ef.to_str());
        }

        println!("\nDoes the skill have another {skilltype} effect? n/no: no, anything else: yes");
        let ans = read_str();
        if ans == "n" || ans == "no" {
            break;
        }
    }

    effects
}

fn build_heals(mut sk: AdventurerSkill) -> AdventurerSkill {
    println!("Which type of healing effects does the skill have? (enter applicable separated by spaces, e.g. '1 2')");
    println!("1: HP healing");
    println!("2: MP healing");
    let ans = read_multinum();

    if ans.contains(&1) {
        let heal = Heal {
            target: get_heal_target(Attribute::Heal),
            heal_type: Attribute::Heal,
            modifier: get_hp_heal_modifier(),
        };
        sk.hp_heal = Some(heal);
    }
    if ans.contains(&2) {
        let heal = Heal {
            target: get_heal_target(Attribute::MPHeal),
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

// Combat Skill building
pub fn build_nameless_skill(is_aa: bool, has_aa: &mut bool) -> AdventurerSkill {
    let mut sk = AdventurerSkill::new();

    println!("\nWhich of the following effects does the skill have? (enter applicable separated by spaces, e.g. '2 4 5')");
    println!("1: Damaging effect (e.g '[Foe] Hi Fire P.Attack')");
    println!("2: Buffs or Debuffs, including HP Regen skills (e.g. '[Self] +80% Str. /3 turns')");
    println!("3: Buff or Debuff Removal (e.g. '[Foes] removes Str. Buffs')");
    println!("4: Buff or Debuff turn effect (e.g. '[Self] Status Debuff-2 turns')");
    println!("5: Nulls, for attacks or ailments");
    println!("6: HP/MP Healing skills (HP heal or MP heal, NOT HP Regen or Life Steal!)");
    println!("7: Ailments (e.g. '[Foes] 35% Sleep')");
    println!("8: Ailment cure");
    println!("9: Kill resist (e.g. '[Allies] Avoids K.O x1 only when HP >= 10%)");
    if !is_aa {
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
    if ans.contains(&10) && !is_aa {
        println!();
        *has_aa = true;
        sk.additional_action = Some(get_additional_action_quantity());
    }

    sk
}

fn build_speedless_skill(is_aa: bool, has_aa: &mut bool) -> AdventurerSkill {
    let name = get_adv_skill_name();
    let mut sk = build_nameless_skill(is_aa, has_aa);
    sk.name = name;

    sk
}

fn build_sa(additional_actions: &mut Vec<AdventurerSkill>) -> AdventurerSkill {
    println!("\n\n\nLet's build the unit's SA");
    let mut has_aa = false;
    let sk = build_speedless_skill(false, &mut has_aa);
    handle_additional_action(&sk, additional_actions, &mut has_aa);
    sk
}

fn build_adv_skill(has_aa: &mut bool) -> AdventurerSkill {
    let mut sk = build_speedless_skill(false, has_aa);
    sk.speed = get_skill_speed();
    sk
}

fn build_adv_skills(additional_actions: &mut Vec<AdventurerSkill>) -> Vec<AdventurerSkill> {
    let mut skills = Vec::new();
    let mut has_aa = false;
    for i in 1..4 {
        println!("\n\n\nLet's build the {}. regular skill", i);
        skills.push(build_adv_skill(&mut has_aa));
        handle_additional_action(skills.last().unwrap(), additional_actions, &mut has_aa);
    }
    skills
}

fn handle_additional_action(
    last_skill: &AdventurerSkill,
    additional_actions: &mut Vec<AdventurerSkill>,
    has_aa: &mut bool,
) {
    if *has_aa {
        println!("\n\n\nLet's build that skill's additional action");
        let mut aa = build_additional_action();
        aa.name = last_skill.name.clone();
        additional_actions.push(aa);
        *has_aa = false;
    }
}

fn build_additional_action() -> AdventurerSkill {
    let sk = build_nameless_skill(true, &mut false);
    sk
}

// Development skills
fn build_dev_skill(adv: &Adventurer) -> DevelopmentSkill {
    let mut dev = DevelopmentSkill::new();
    println!(
        "\nPlease enter the full name of the development skill (e.g. 'Light Manifestation: H')"
    );
    let ans = read_nonempty_str();
    let dev_type = DevelopmentSkillType::str_to_type(&ans);
    if ans.contains(":") {
        let rk = ans.split(":").nth(1).unwrap().trim();
        dev.rank = SkillRank::str_to_type(rk);
    }

    use DevelopmentSkillType::*;
    dev.effect = match dev_type {
        // Unrecognized development skill
        Unknown(title) => Unknown(title),
        // No parameters
        Encouragement | Blessing | Flashback | LiarisFreese => dev_type,
        // Only one u32
        Bravery(_) => Bravery(get_dev_modifier()),
        Hex(_) => Hex(get_dev_modifier()),
        MartialArts(_) => MartialArts(get_dev_modifier()),
        Tattletale(_) => Tattletale(get_dev_modifier()),
        FightingSpirit(_) => FightingSpirit(get_dev_modifier()),
        BravePerformance(_) => BravePerformance(get_dev_modifier()),
        BattleBravery(_) => BattleBravery(get_dev_modifier()),
        Rigid(_) => Rigid(get_dev_modifier()),
        Forestall(_) => Forestall(get_dev_modifier()),
        BattleArts(_) => BattleArts(get_dev_modifier()),
        Concentrate(_) => Instinct(get_dev_modifier()),
        Instinct(_) => Instinct(get_dev_modifier()),
        Climb(_) => Climb(get_dev_modifier()),
        Crush(_) => Crush(get_dev_modifier()),
        FistStrike(_) => FistStrike(get_dev_modifier()),
        Mage(_) => Mage(get_dev_modifier()),
        MindsEye(_) => MindsEye(get_dev_modifier()),
        Acceleration(_) => Acceleration(get_dev_modifier()),
        Hunter(_) => Hunter(get_dev_modifier()),
        Crafter(_) => Crafter(get_dev_modifier()),
        Protection(_) => Protection(get_dev_modifier()),
        MagicResistance(_) => MagicResistance(get_dev_modifier()),
        StatusResist(_) => StatusResist(get_dev_modifier()),
        AbnormalResistance(_) => AbnormalResistance(get_dev_modifier()),
        Solid(_) => Solid(get_dev_modifier()),
        Strike(_) => Strike(get_dev_modifier()),
        PiercingStrike(_) => PiercingStrike(get_dev_modifier()),
        TrueStrike(_) => TrueStrike(get_dev_modifier()),
        CounterAttack(_) => CounterAttack(get_dev_modifier()),
        Bloom(_) => Bloom(get_dev_modifier()),
        SpiritHealing(_) => SpiritHealing(get_dev_modifier()),
        Luck(_) => Luck(get_dev_modifier()),
        // Others
        Killer(x) => Killer(x),
        WillOf(_, _) => WillOf(adv.element.clone(), adv.damage_type.clone()),
        Manifestation(_, _, _) => Manifestation(
            adv.element.clone(),
            adv.damage_type.clone(),
            get_dev_modifier(),
        ),
        Resistance(_, _) => Resistance(adv.element.effective_against(), get_dev_modifier()),
    };

    if let Unknown(_) = dev.effect {
        println!("Name not recognized. A suitable section in the JSON will be created, but you will have to fill it out yourself.")
    }

    dev
}

fn build_dev_skills(adv: &Adventurer) -> Vec<DevelopmentSkill> {
    let mut devs = Vec::new();
    println!("\n\n\nLet's build the development skills");
    loop {
        devs.push(build_dev_skill(&adv));

        println!("\n\nThe unit currently has the following development skills:");
        for dev in &devs {
            println!("{}", dev.to_str());
        }
        println!("Does the unit have another development skill? n/no: no, anything else: yes");
        let ans = read_str();
        if ans == "n" || ans == "no" {
            println!();
            break;
        }
    }
    devs
}

// Combination
pub fn build_adv(unit: Unit) -> Adventurer {
    let mut adv = Adventurer::new(unit);
    adv.adventurer_type = get_adventurer_type();
    adv.damage_type = get_dmg_type(&adv.adventurer_type);
    adv.element = get_element();

    let mut additional_actions = Vec::new();
    adv.sa = build_sa(&mut additional_actions);

    adv.reg_skills = build_adv_skills(&mut additional_actions);
    adv.additional_actions = additional_actions;

    adv.dev_skills = build_dev_skills(&adv);

    adv
}
