use crate::enums::*;
use std::io::stdin;

// Helper functions
pub fn read_num() -> i32 {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Error!");
    let input: i32 = match input.trim().parse() {
        Ok(result) => result,
        Err(_) => panic!("Error reading input"),
    };
    input
}

pub fn read_str() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error!");
    input.trim().to_string()
}

// getters
pub fn get_unit_type() -> UnitType {
    println!("Adventurer or Assist? (1: Adventurer, 2: Assist)");
    let input = read_num();
    let unit_type = match input {
        1 => UnitType::Adventurer,
        2 => UnitType::Assist,
        _ => panic!("Please enter either 1 or 2"),
    };
    unit_type
}

pub fn get_title() -> String {
    println!(
        "What is the unit's title? (e.g. \"Decisive Will\" in \"[Decisive Will] Bell Cranel\")"
    );
    let title = read_str();
    title
}

pub fn get_name() -> String {
    println!("What is the unit's name? (e.g. \"Bell Cranel\" in \"[Decisive Will] Bell Cranel\")");
    let name = read_str();
    name
}

pub fn get_attack_type() -> AttackType {
    println!("Which type is the adventurer? (1: Physical, 2: Magic, 3: Balance)");
    let at_type = read_num();
    let at_type = match at_type {
        1 => AttackType::Physical,
        2 => AttackType::Magic,
        3 => AttackType::Balance,
        _ => panic!("Please enter a number from 1 to 3"),
    };
    at_type
}

pub fn get_stars() -> i32 {
    println!("How many stars does the unit have? (1-4)");
    let stars = read_num();
    if stars > 4 || stars < 1 {
        panic!("Please enter a number from 1 to 4")
    }
    stars
}

pub fn get_limited() -> bool {
    println!("Is the unit time-limited? (n/no, y/yes)");
    let limited = read_str();
    let limited = match limited.as_str() {
        "n" => false,
        "no" => false,
        "yes" => true,
        "y" => true,
        _ => panic!("Please enter 'n, 'no', 'y' or 'yes'"),
    };
    limited
}

pub fn get_six_stats(stat: &str) -> Vec<String> {
    let mut l: Vec<String> = Vec::new();
    println!("How much {} does the unit have at base?", stat);
    l.push(read_num().to_string());
    for num in 1..5 {
        println!("And at +{}?", num);
        l.push(read_num().to_string());
    }
    println!("And at MLB?");
    l.push(read_num().to_string());
    l
}

// Assist (skill) stuff
pub fn get_ass_skill_name() -> String {
    println!("What is the name of the assist's skill? (without '+' at the end)");
    let name = read_str();
    name
}

// Assist effect stuff
pub fn get_ass_target(attr: &Attribute) -> Target {
    let target = match attr {
        Attribute::Sleep
        | Attribute::Stun
        | Attribute::Seal
        | Attribute::Slow
        | Attribute::Taunt
        | Attribute::Poison => Target::Foes,
        _ => {
            println!("Who is the effect's target? (1: Self, 2: Allies, 3: Foes)");
            let input = read_num();
            match input {
                1 => Target::Auto,
                2 => Target::Allies,
                3 => Target::Foes,
                _ => panic!("Please enter a number from 1 to 3"),
            }
        }
    };

    target
}

pub fn get_ass_modifier(attr: &Attribute) -> i32 {
    let modi = match attr {
        Attribute::AoEResistance | Attribute::STResistance => {
            println!(
                "By how many percent does it increase or decrease damage? (leave out the '%' sign)"
            );
            let input = -1 * read_num();
            input
        }
        Attribute::MPRegen => {
            println!("How many MP does it regen per turn?");
            let input = read_num();
            if input <= 0 {
                panic!("Please enter a number greater than 0")
            }
            input
        }
        Attribute::Sleep
        | Attribute::Stun
        | Attribute::Seal
        | Attribute::Slow
        | Attribute::Taunt
        | Attribute::Poison => {
            println!("What is the chance to activate the ailment? (leave out the '%' sign)");
            let input = read_num();
            if input <= 0 {
                panic!("Please enter a number greater than 0");
            }
            input
        }
        Attribute::NullPhysical | Attribute::NullMagical | Attribute::NullAilment => {
            println!("How many nulls does it add?");
            let input = read_num();
            if input <= 0 {
                panic!("Please enter a number greater than 0");
            }
            input
        }
        Attribute::BuffTurns | Attribute::DebuffTurns => {
            println!("By how many turns does it lengthen buffs/debuffs? (negative number for decreasing turns");
            let input = read_num();
            input
        }
        _ => {
            println!("By how many percent does it increase/decrease the stat? (negative number for decrease. Leave out the '%' sign)");
            let input = read_num();
            input
        }
    };

    modi
}

// Get assist skill effect attribute, main menu plus 7 submenus
pub fn get_ass_attribute() -> Attribute {
    println!("\nWhat effect does it have/what attribute does it affect?");
    println!("1: Base stats (Strength etc.)");
    println!("2: Resistances");
    println!("3: All Targets or Single Target Damage");
    println!("4: Elemental damage");
    println!("5. Healing (HP or MP)");
    println!("6: Ailments");
    println!("7: Nulls or Buff/Debuff turns");
    println!("8: Misc (Guard Rate, SA charge etc.");
    let submenu = read_num();
    match submenu {
        1 => return get_ass_base_attr(),
        2 => return get_ass_res_attr(),
        3 => return get_ass_aoe_st_attr(),
        4 => return get_ass_el_attr(),
        5 => return get_ass_heal_attr(),
        6 => return get_ass_ail_attr(),
        7 => return get_ass_null_turns_attr(),
        8 => return get_ass_misc_attr(),
        _ => panic!("Please enter a number from 1 to 7"),
    }
}

fn get_ass_base_attr() -> Attribute {
    println!("Which attribute does it affect?\n1: Strength\t2: Magic\n3: Agility\t4: Dexterity\t5: Endurance");
    let attr = read_num();
    match attr {
        1 => Attribute::Strength,
        2 => Attribute::Magic,
        3 => Attribute::Agility,
        4 => Attribute::Dexterity,
        5 => Attribute::Endurance,
        _ => panic!("Please enter a number from 1 to 5"),
    }
}

fn get_ass_res_attr() -> Attribute {
    println!(
        "Which resistance does it affect?\n1: Physical Resistance\t2: Magic Resistance
        3: Light Resistance\t4: Dark Resistance\t5: Fire Resistance\t6: Water Resistance
        7: Thunder Resistance\t8: Earth Resistance\t9: Wind Resistance
        10: Sleep Resistance\t11: Stun Resistance\t12: Seal Resistance
        13: Slow Resistance\t14: Taunt Resistance\t15: Poison Resistance"
    );
    let attr = read_num();
    match attr {
        1 => Attribute::PhysicalResistance,
        2 => Attribute::MagicResistance,
        3 => Attribute::LightResistance,
        4 => Attribute::DarkResistance,
        5 => Attribute::FireResistance,
        6 => Attribute::WaterResistance,
        7 => Attribute::ThunderResistance,
        8 => Attribute::EarthResistance,
        9 => Attribute::WindResistance,
        10 => Attribute::SleepResist,
        11 => Attribute::StunResist,
        12 => Attribute::SealResist,
        13 => Attribute::SlowResist,
        14 => Attribute::TauntResist,
        15 => Attribute::PoisonResist,
        _ => panic!("Please enter a number from 1 to 15"),
    }
}

fn get_ass_aoe_st_attr() -> Attribute {
    println!("What type of damage does it affect? 1: All Targets Damage\t2: Single Target Damage");
    let attr = read_num();
    match attr {
        1 => Attribute::AoEResistance,
        2 => Attribute::STResistance,
        _ => panic!("Please enter either 1 or 2"),
    }
}

fn get_ass_el_attr() -> Attribute {
    println!(
        "Which element's damage does it affect?
        1: Light\t2: Dark\t3: Fire\t4: Water\t5: Thunder\t6: Earth\t7: Wind"
    );
    let attr = read_num();
    match attr {
        1 => Attribute::LightDamage,
        2 => Attribute::DarkDamage,
        3 => Attribute::FireDamage,
        4 => Attribute::WaterDamage,
        5 => Attribute::ThunderDamage,
        6 => Attribute::EarthDamage,
        7 => Attribute::WindDamage,
        _ => panic!("Please enter a number from 1 to 15"),
    }
}

fn get_ass_heal_attr() -> Attribute {
    println!(
        "How does it affect healing/what sort of healing does it do?
        1: HP Heal\t2: HP Regen\t3: MP Regen\t4: Healing Rate"
    );
    let attr = read_num();
    match attr {
        1 => Attribute::Heal,
        2 => Attribute::HPRegen,
        3 => Attribute::MPRegen,
        4 => Attribute::HealRate,
        _ => panic!("Please enter a number from 1 to 4"),
    }
}

fn get_ass_ail_attr() -> Attribute {
    println!(
        "What ailment does it inflict?
        1: Sleep\t2: Stun\t3: Seal\t4: Slow\t5: Taunt\t6: Poison"
    );
    let attr = read_num();
    match attr {
        1 => Attribute::Sleep,
        2 => Attribute::Stun,
        3 => Attribute::Seal,
        4 => Attribute::Slow,
        5 => Attribute::Taunt,
        6 => Attribute::Poison,
        _ => panic!("Please enter a number from 1 to 6"),
    }
}

fn get_ass_null_turns_attr() -> Attribute {
    println!(
        "What does it affect?
        1: Physical Nulls\t2: Magic Nulls\t3: Ailment Nulls\t4: Buff Turns\t5: Debuff Turns"
    );
    let attr = read_num();
    match attr {
        1 => Attribute::NullPhysical,
        2 => Attribute::NullMagical,
        3 => Attribute::NullAilment,
        4 => Attribute::BuffTurns,
        5 => Attribute::DebuffTurns,
        _ => panic!("Please enter a number from 1 to 5"),
    }
}

fn get_ass_misc_attr() -> Attribute {
    println!(
        "What does it affect?\n1: Guard Rate, 2: Counter Rate, 3: Critical Rate, 4: Penetration Rate, 5: SA Charge Gauge Gain"
    );
    let attr = read_num();
    match attr {
        1 => Attribute::GuardRate,
        2 => Attribute::CounterRate,
        3 => Attribute::CriticalRate,
        4 => Attribute::PenetrationRate,
        5 => Attribute::SACharge,
        _ => panic!("Please enter a number from 1 to 5"),
    }
}
