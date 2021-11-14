use crate::enums::*;
use crate::get_attributes::*;
use std::convert::TryInto;

pub fn get_adventurer_type() -> AdventurerType {
    get_numeric_option(
        "Which type is the adventurer?",
        vec![
            AdventurerType::Physical,
            AdventurerType::Magic,
            AdventurerType::Balance,
        ],
        1,
    )
}

pub fn get_element() -> Element {
    get_numeric_option(
        "What is the adventurer's element?",
        vec![
            Element::None,
            Element::Light,
            Element::Dark,
            Element::Fire,
            Element::Water,
            Element::Thunder,
            Element::Earth,
            Element::Wind,
        ],
        0,
    )
}

pub fn get_dmg_type(at: &AdventurerType) -> DamageType {
    match at {
        AdventurerType::Physical => DamageType::Physical,
        AdventurerType::Magic => DamageType::Magic,
        AdventurerType::Balance => get_numeric_option(
            "What type of damage does the unit do?",
            vec![DamageType::Physical, DamageType::Magic],
            0,
        ),
    }
}

// Adventurer Skill
pub fn get_adv_skill_name() -> String {
    println!("What is the name of the skill? (without '+' at the end)");
    let name = read_str();
    name
}

pub fn get_skill_speed() -> Speed {
    get_numeric_option(
        "What is the skill's speed?",
        vec![Speed::None, Speed::Fast, Speed::Slow],
        1,
    )
}

// Adventurer Effects
// Damaging Effect
pub fn get_dmg_target() -> Target {
    get_numeric_option(
        "Who is the attack's target?",
        vec![Target::Foes, Target::Foe],
        1,
    )
}

pub fn get_dmg_mod() -> SkillModifier {
    get_numeric_option(
        "What's the damage modifier?",
        vec![
            SkillModifier::Low,
            SkillModifier::Medium,
            SkillModifier::High,
            SkillModifier::Super,
            SkillModifier::Ultra,
        ],
        1,
    )
}

pub fn get_temp_boost() -> TempBoost {
    get_numeric_option(
        "Which temporary boost does the attack have?",
        vec![TempBoost::Boost, TempBoost::GreatBoost],
        1,
    )
}

pub fn get_rate_buff() -> RateAttribute {
    get_numeric_option(
        "Which rate does the rate buff affect?",
        vec![
            RateAttribute::Unguard,
            RateAttribute::Uncounter,
            RateAttribute::Critical,
            RateAttribute::Penetration,
        ],
        1,
    )
}

pub fn get_lifesteal() -> u32 {
    get_chance("How much life steal does the attack have? (leave out the '%' sign)")
}

// Per Skill Effect Damage Boosts
pub fn get_per_effect_boost_attr() -> Attribute {
    get_numeric_option(
        "On which attribute does the damage buff depend?",
        vec![
            Attribute::Strength,
            Attribute::Magic,
            Attribute::Agility,
            Attribute::Dexterity,
            Attribute::Endurance,
            Attribute::PhysicalResistance,
            Attribute::MagicResistance,
            Attribute::AoEResistance,
            Attribute::STResistance,
            Attribute::LightDamage,
            Attribute::DarkDamage,
            Attribute::FireDamage,
            Attribute::WaterDamage,
            Attribute::ThunderDamage,
            Attribute::EarthDamage,
            Attribute::WindDamage,
            Attribute::GuardRate,
            Attribute::CounterRate,
            Attribute::CriticalRate,
            Attribute::PenetrationRate,
        ],
        1,
    )
}

pub fn get_per_effect_boost_source() -> Target {
    get_numeric_option(
        "What does the damage buff depend on?",
        vec![Target::Auto, Target::Foe],
        1,
    )
}
pub fn get_per_effect_boost_mod() -> u32 {
    get_u32("By how much does each effect increase the damage?")
}

// Buffs
pub fn get_buff_target() -> Target {
    get_numeric_option(
        "Who is the buff's/debuff's target?",
        vec![Target::Auto, Target::Allies, Target::Foe, Target::Foes],
        1,
    )
}

pub fn get_buff_attribute() -> Attribute {
    println!("\nWhat attribute does it affect?");
    println!("1: Base stats (Strength etc.)");
    println!("2: Resistances");
    println!("3: All Targets or Single Target Damage");
    println!("4: Elemental damage");
    println!("5: Rate buffs (Guard Rate etc.");
    println!("6: HP Regen");
    println!("7: S.A Gauge Charge gain");

    let ans = read_num();
    match ans {
        1 => return get_attr_base(),
        2 => return get_attr_res(),
        3 => return get_attr_aoe_st(),
        4 => return get_attr_el(),
        5 => return get_adv_attr_rate(),
        6 => return Attribute::HPRegen,
        7 => return Attribute::SACharge,
        _ => panic!("Please enter a number from 1 to 7"),
    }
}

fn get_adv_attr_rate() -> Attribute {
    get_numeric_option(
        "Which rate does it affect?",
        vec![
            Attribute::GuardRate,
            Attribute::CounterRate,
            Attribute::CriticalRate,
            Attribute::PenetrationRate,
        ],
        1,
    )
}

pub fn get_buff_mod(attr: &Attribute) -> i32 {
    match attr {
        Attribute::HPRegen => println!("How many percent does it heal? (leave out the '%' sign)"),
        _ => println!("By how many percent does it increase/decrease the stat? (negative number for decrease. Leave out the '%' sign)")
    }
    let ans = read_num();
    match attr {
        Attribute::AoEResistance | Attribute::STResistance => -1 * ans,
        _ => ans,
    }
}

pub fn get_duration() -> u32 {
    println!("How many turns does the effect last?");
    let ans = read_num();
    if ans <= 0 {
        panic!("Please enter a number greater than 0");
    }
    ans.try_into().unwrap()
}

// Buff Removal
pub fn get_buff_removal_target() -> Target {
    get_numeric_option(
        "Whose buffs/debuffs are removed?",
        vec![Target::Auto, Target::Allies, Target::Foe, Target::Foes],
        1,
    )
}

pub fn get_buff_removal_attribute(kind: &BuffType) -> Attribute {
    println!("\nWhat attribute does it affect?");
    println!("1: Base stats (Strength etc.)");
    println!("2: Resistances");
    println!("3: All Targets or Single Target Damage");
    println!("4: Elemental damage");
    println!("5: Rate buffs (Guard Rate etc.");
    println!("6: HP Regen");
    println!("7: All");

    let ans = read_num();
    match ans {
        1 => return get_attr_base(),
        2 => return get_attr_res_no_ailment(),
        3 => return get_attr_aoe_st(),
        4 => return get_attr_el(),
        5 => return get_adv_attr_rate(),
        6 => return Attribute::HPRegen,
        7 => match kind {
            BuffType::Buff => Attribute::BuffTurns,
            BuffType::Debuff => Attribute::DebuffTurns,
        },
        _ => panic!("Please enter a number from 1 to 7"),
    }
}

pub fn get_buff_removal_kind() -> BuffType {
    get_numeric_option(
        "What does it remove?",
        vec![BuffType::Buff, BuffType::Debuff],
        1,
    )
}

// Buff Turns
pub fn get_buff_turns_target() -> Target {
    get_numeric_option(
        "Whose buffs/debuff turns are affected?",
        vec![Target::Auto, Target::Allies, Target::Foe, Target::Foes],
        1,
    )
}

pub fn get_buff_turns_kind() -> BuffType {
    get_numeric_option(
        "Which ones are affected?",
        vec![BuffType::Buff, BuffType::Debuff],
        1,
    )
}

pub fn get_buff_turns_number(kind: &BuffType) -> i32 {
    println!(
        "By how many turns does it lengthen/shorten {}s? (negative number for decrease?",
        kind.to_str()
    );
    let ans = read_num();
    if ans == 0 {
        panic!("Please enter a number other than 0");
    }
    ans.try_into().unwrap()
}

// Nulls
pub fn get_null_target() -> Target {
    get_numeric_option(
        "Who does it apply nulls to?",
        vec![Target::Auto, Target::Allies],
        1,
    )
}

pub fn get_null_kind() -> Attribute {
    get_numeric_option(
        "What type of nulls does it apply?",
        vec![
            Attribute::NullPhysical,
            Attribute::NullMagical,
            Attribute::NullAilment,
        ],
        1,
    )
}

pub fn get_null_amount() -> u32 {
    get_u32("How many nulls does it apply?")
}

pub fn get_null_chance() -> u32 {
    get_chance(
        "What is the chance to apply the null(s)? (100 if guaranteed, leave out the '%' sign)",
    )
}

// Heals
pub fn get_heal_target() -> Target {
    get_numeric_option(
        "Who is the heal applied to?",
        vec![Target::Auto, Target::Allies],
        1,
    )
}

pub fn get_hp_heal_modifier() -> HealModifier {
    println!("Is the heal modifier-based (e.g. 'Hi Heal') or percentage-based? 1: percentage-based, 2: modifier-based");
    let ans = read_num();
    if ans == 1 {
        return HealModifier::Numeric(get_chance(
            "What percentage of HP does it heal? (leave out the '%' sign)",
        ));
    } else if ans == 2 {
        let ans = get_numeric_option(
            "What is the modifier?",
            vec![
                SkillModifier::Low,
                SkillModifier::Medium,
                SkillModifier::High,
                SkillModifier::Super,
                SkillModifier::Ultra,
            ],
            1,
        );
        return HealModifier::SkillModifier(ans);
    } else {
        panic!("Please enter either 0 or 1")
    }
}

pub fn get_mp_heal_modifier() -> HealModifier {
    HealModifier::Numeric(get_chance(
        "What percentage of MP does it heal? (leave out the '%' sign)",
    ))
}

// Ailments
pub fn get_ail_target() -> Target {
    get_numeric_option(
        "Who is the ailment applied to?",
        vec![Target::Foes, Target::Foe],
        1,
    )
}

pub fn get_ail_kind() -> Attribute {
    get_numeric_option(
        "What ailment is applied?",
        vec![
            Attribute::Sleep,
            Attribute::Stun,
            Attribute::Seal,
            Attribute::Slow,
            Attribute::Taunt,
            Attribute::Poison,
        ],
        1,
    )
}

pub fn get_ail_chance() -> u32 {
    get_chance("What is the chance to apply the ailment(s)? (leave out the '%' sign)")
}
