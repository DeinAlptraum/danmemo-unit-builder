use crate::enums::*;
use crate::get_attributes::*;
use std::convert::TryInto;

pub fn get_adventurer_type() -> AdventurerType {
    get_numeric_option(
        "\nWhich type is the adventurer?",
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
        "\nWhat is the adventurer's element?",
        vec![
            Element::None,
            Element::Fire,
            Element::Water,
            Element::Thunder,
            Element::Earth,
            Element::Wind,
            Element::Light,
            Element::Dark,
        ],
        0,
    )
}

pub fn get_dmg_type(at: &AdventurerType) -> DamageType {
    match at {
        AdventurerType::Physical => DamageType::Physical,
        AdventurerType::Magic => DamageType::Magic,
        AdventurerType::Balance => get_numeric_option(
            "\nWhat type of damage does the unit do?",
            vec![DamageType::Physical, DamageType::Magic],
            1,
        ),
    }
}

// Adventurer Skill
pub fn get_adv_skill_name() -> String {
    println!("What is the name of the skill?");
    let name = read_nonempty_str();
    name
}

pub fn get_skill_speed() -> Speed {
    get_numeric_option(
        "\nWhat is the skill's speed?",
        vec![Speed::None, Speed::Fast, Speed::Slow],
        1,
    )
}

// Adventurer Effects
// Damaging Effect
pub fn get_dmg_target() -> Target {
    get_numeric_option(
        "\nWho is the attack's target?",
        vec![Target::Foes, Target::Foe],
        1,
    )
}

pub fn get_dmg_mod() -> SkillModifier {
    get_numeric_option(
        "\nWhat's the damage modifier?",
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
        "\nWhich temporary boost does the attack have?",
        vec![TempBoost::Boost, TempBoost::GreatBoost],
        1,
    )
}

pub fn get_rate_buff_attr() -> RateAttribute {
    get_numeric_option(
        "\nWhich rate does the rate buff affect?",
        vec![
            RateAttribute::Unguard,
            RateAttribute::Uncounter,
            RateAttribute::Critical,
            RateAttribute::Penetration,
        ],
        1,
    )
}

pub fn get_rate_buff_mod() -> SkillModifier {
    get_numeric_option(
        "\nWhat is the rate buff's modifier?",
        vec![SkillModifier::High, SkillModifier::Ultra],
        1,
    )
}

pub fn get_lifesteal() -> u32 {
    get_chance("\nHow much life steal does the attack have? (leave out the '%' sign)")
}

// Per Skill Effect Damage Boosts
pub fn get_per_effect_boost_attrs() -> Vec<Attribute> {
    println!("\nOn which attributes does the damage buff depend? (enter applicable separated by spaces, e.g. '2 3 12')");
    println!("1: Strength, 2: Magic, 3: Endurance, 4: Agility, 5: Dexterity");
    println!("6: P.Resist, 7: M.Resist, 8: Dmg. received (Attack Type: All Targets), 9: Dmg. received (Attack Type: Single Target)");
    println!("10: Fire Dmg., 11: Water Dmg., 12: Thunder Dmg., 13: Earth Dmg.");
    println!("14: Wind Dmg., 15: Light Dmg., 16: Dark Dmg.");
    println!("17: Fire Resist, 18: Water Resist, 19: Thunder Resist, 20: Earth Resist");
    println!("21: Wind Resist, 22: Light Resist, 23: Dark Resist");
    println!("24: Critical Rate, 25: Penetration Rate, 26: Counter Rate, 27: Guard Rate");
    println!("28: Sleep, 29: Stun, 30: Seal, 31: Slow, 32: Taunt, 33: Poison, 34: Charm");
    println!("35: HP Regen, 36: Heal Amount, 37: SA Gauge Charge Gain");
    'outer: loop {
        let ans = read_multinum();
        let mut res = Vec::new();
        for num in ans {
            match num {
                1 => res.push(Attribute::Strength),
                2 => res.push(Attribute::Magic),
                3 => res.push(Attribute::Endurance),
                4 => res.push(Attribute::Agility),
                5 => res.push(Attribute::Dexterity),
                6 => res.push(Attribute::PhysicalResistance),
                7 => res.push(Attribute::MagicResistance),
                8 => res.push(Attribute::AoEResistance),
                9 => res.push(Attribute::STResistance),
                10 => res.push(Attribute::FireDamage),
                11 => res.push(Attribute::WaterDamage),
                12 => res.push(Attribute::ThunderDamage),
                13 => res.push(Attribute::EarthDamage),
                14 => res.push(Attribute::WindDamage),
                15 => res.push(Attribute::LightDamage),
                16 => res.push(Attribute::DarkDamage),
                17 => res.push(Attribute::FireResistance),
                18 => res.push(Attribute::WaterResistance),
                19 => res.push(Attribute::ThunderResistance),
                20 => res.push(Attribute::EarthResistance),
                21 => res.push(Attribute::WindResistance),
                22 => res.push(Attribute::LightResistance),
                23 => res.push(Attribute::DarkResistance),
                24 => res.push(Attribute::CriticalRate),
                25 => res.push(Attribute::PenetrationRate),
                26 => res.push(Attribute::CounterRate),
                27 => res.push(Attribute::GuardRate),
                28 => res.push(Attribute::Sleep),
                29 => res.push(Attribute::Stun),
                30 => res.push(Attribute::Seal),
                31 => res.push(Attribute::Slow),
                32 => res.push(Attribute::Taunt),
                33 => res.push(Attribute::Poison),
                34 => res.push(Attribute::Charm),
                35 => res.push(Attribute::HPRegen),
                36 => res.push(Attribute::HealRate),
                37 => res.push(Attribute::SACharge),
                _ => {
                    println!("Please enter a number from 1 to 27");
                    continue 'outer;
                }
            }
        }
        return res;
    }
}

pub fn get_per_effect_boost_source() -> Target {
    get_numeric_option(
        "\nWhose buff/debuff does the damage buff depend on?",
        vec![Target::Auto, Target::Foe],
        1,
    )
}

pub fn get_per_effect_boost_kind() -> BuffType {
    get_numeric_option(
        "\nWhat does the damage buff depend on?",
        vec![BuffType::Buff, BuffType::Debuff],
        1,
    )
}

pub fn get_per_effect_boost_mod() -> u32 {
    get_u32("\nBy how much does each effect increase the damage? (leave out the '%' sign)")
}

// Buffs
pub fn get_buff_target() -> Target {
    get_numeric_option(
        "\nWho is the buff/debuff's target?",
        vec![Target::Auto, Target::Allies, Target::Foe, Target::Foes],
        1,
    )
}

pub fn get_buff_attribute() -> Attribute {
    println!("\nWhat attribute does it affect?");
    println!("1: Base stats (Strength, Agility etc.)");
    println!("2: Resistances");
    println!("3: All Targets or Single Target Damage");
    println!("4: Elemental damage");
    println!("5: Rate buffs (Guard Rate etc.)");
    println!("6: HP Regen");
    println!("7: Heal Buff (increase to amount healed by other skills)");
    println!("8: S.A Gauge Charge gain");

    loop {
        let ans = read_num();
        match ans {
            1 => return get_attr_base(),
            2 => return get_attr_res(),
            3 => return get_attr_aoe_st(),
            4 => return get_attr_el(),
            5 => return get_adv_attr_rate(),
            6 => return Attribute::HPRegen,
            7 => return Attribute::HealRate,
            8 => return Attribute::SACharge,
            _ => {
                println!("Please enter a number from 1 to 8");
                continue;
            }
        }
    }
}

fn get_adv_attr_rate() -> Attribute {
    get_numeric_option(
        "\nWhich rate does it affect?",
        vec![
            Attribute::CriticalRate,
            Attribute::PenetrationRate,
            Attribute::CounterRate,
            Attribute::GuardRate,
        ],
        1,
    )
}

pub fn get_buff_mod(attr: &Attribute) -> i32 {
    match attr {
        Attribute::HPRegen => println!("\nHow many percent does it heal? (leave out the '%' sign)"),
        _ => println!("\nBy how many percent does it increase/decrease the stat? (negative number for decrease. Leave out the '%' sign)")
    }
    let ans = read_num();
    match attr {
        Attribute::AoEResistance | Attribute::STResistance => -1 * ans,
        _ => ans,
    }
}

pub fn get_duration() -> u32 {
    loop {
        println!("\nHow many turns does the effect last?");
        let ans = read_num();
        if ans <= 0 {
            println!("Please enter a number greater than 0");
            continue;
        }
        return ans.try_into().unwrap();
    }
}

// Buff Removal
pub fn get_buff_removal_target() -> Target {
    get_numeric_option(
        "\nWhose buffs/debuffs are removed?",
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
    println!("7: Heal Buff (increase to amount healed by other skills)");
    println!("8: All");

    loop {
        let ans = read_num();
        match ans {
            1 => return get_attr_base(),
            2 => return get_attr_res(),
            3 => return get_attr_aoe_st(),
            4 => return get_attr_el(),
            5 => return get_adv_attr_rate(),
            6 => return Attribute::HPRegen,
            7 => return Attribute::Heal,
            8 => match kind {
                BuffType::Buff => return Attribute::BuffTurns,
                BuffType::Debuff => return Attribute::DebuffTurns,
            },
            _ => {
                println!("Please enter a number from 1 to 8");
                continue;
            }
        }
    }
}

pub fn get_buff_removal_kind() -> BuffType {
    get_numeric_option(
        "\nWhat does it remove?",
        vec![BuffType::Buff, BuffType::Debuff],
        1,
    )
}

// Buff Turns
pub fn get_buff_turns_target() -> Target {
    get_numeric_option(
        "\nWhose buff/debuff turns are affected?",
        vec![Target::Auto, Target::Allies, Target::Foe, Target::Foes],
        1,
    )
}

pub fn get_buff_turns_kind() -> BuffType {
    get_numeric_option(
        "\nWhich ones are affected?",
        vec![BuffType::Buff, BuffType::Debuff],
        1,
    )
}

pub fn get_buff_turns_number(kind: &BuffType) -> i32 {
    println!(
        "\nBy how many turns does it lengthen/shorten {}s? (negative number for decrease)",
        kind.to_str()
    );

    loop {
        let ans = read_num();
        if ans == 0 {
            println!("Please enter a number other than 0");
            continue;
        }
        return ans.try_into().unwrap();
    }
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
        "\nWhat type of nulls does it apply?",
        vec![
            Attribute::NullPhysical,
            Attribute::NullMagical,
            Attribute::NullAilment,
        ],
        1,
    )
}

pub fn get_null_amount() -> u32 {
    get_u32("\nHow many nulls does it apply?")
}

pub fn get_null_chance() -> u32 {
    get_chance(
        "\nWhat is the chance to apply the null(s)? (100 if guaranteed, leave out the '%' sign)",
    )
}

// Heals
pub fn get_heal_target(attr: Attribute) -> Target {
    let question = match &attr {
        Attribute::Heal => "\nWho is the HP heal applied to?",
        Attribute::MPHeal => "\nWho is the MP heal applied to?",
        _ => "\nWho is the HP heal applied to?",
    };
    get_numeric_option(question, vec![Target::Auto, Target::Allies], 1)
}

pub fn get_hp_heal_modifier() -> HealModifier {
    println!("\nIs the heal modifier-based (e.g. 'Hi Heal') or percentage-based? 1: modifier-based, 2: percentage-based");

    loop {
        let ans = read_num();
        if ans == 1 {
            let ans = get_numeric_option(
                "\nWhat is the modifier?",
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
        } else if ans == 2 {
            return HealModifier::Numeric(get_chance(
                "\nWhat percentage of HP does it heal? (leave out the '%' sign)",
            ));
        } else {
            println!("Please enter either 1 or 2");
            continue;
        }
    }
}

pub fn get_mp_heal_modifier() -> HealModifier {
    HealModifier::Numeric(get_chance(
        "\nWhat percentage of MP does it heal? (leave out the '%' sign)",
    ))
}

// Kill Resist
pub fn get_kill_resist_target() -> Target {
    get_numeric_option(
        "\nWho is the kill resist applied to?",
        vec![Target::Auto, Target::Allies],
        1,
    )
}

pub fn get_kill_resist_threshold() -> u32 {
    get_chance(
        "\nWhat is the threshold below which the kill resist is removed? (leave out the '%' sign)",
    )
}

// Additional Actions
pub fn get_additional_action_quantity() -> u32 {
    get_u32("\nHow many additional actions does the skill add?")
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
        "\nWhat ailment is applied?",
        vec![
            Attribute::Sleep,
            Attribute::Stun,
            Attribute::Seal,
            Attribute::Slow,
            Attribute::Taunt,
            Attribute::Poison,
            Attribute::Charm,
        ],
        1,
    )
}

pub fn get_ail_chance(ail: &Attribute) -> u32 {
    match ail {
        Attribute::Poison => {
            get_chance("\nWhat is the strength of the poison? (leave out the '%' sign)")
        }
        _ => get_chance("\nWhat is the chance to apply the ailment(s)? (leave out the '%' sign)"),
    }
}

// Development Skills
pub fn get_dev_modifier() -> u32 {
    get_u32("\nWhat is the development skill's percentage-based modifier? (leave out the '%' sign)")
}
