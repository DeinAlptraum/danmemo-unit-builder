use crate::enums::*;
use crate::get_attributes::*;

// Assist Skill
pub fn get_ass_skill_name() -> String {
    println!("What is the name of the assist's skill? (without '+' at the end)");
    let name = read_str();
    name
}

// Assist Effect
pub fn get_ass_target(attr: &Attribute) -> Target {
    match attr {
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
    }
}

pub fn get_ass_modifier(attr: &Attribute) -> i32 {
    let ans;
    match attr {
        Attribute::AoEResistance | Attribute::STResistance => {
            println!(
                "By how many percent does it increase or decrease damage? (leave out the '%' sign)"
            );
            ans = -1 * read_num();
        }
        Attribute::MPRegen => {
            println!("How many MP does it regen per turn?");
            ans = read_num();
            if ans <= 0 {
                panic!("Please enter a number greater than 0");
            }
        }
        Attribute::Sleep
        | Attribute::Stun
        | Attribute::Seal
        | Attribute::Slow
        | Attribute::Taunt
        | Attribute::Poison => {
            println!("What is the chance to activate the ailment? (leave out the '%' sign)");
            ans = read_num();
            if ans <= 0 {
                panic!("Please enter a number greater than 0");
            }
        }
        Attribute::NullPhysical | Attribute::NullMagical | Attribute::NullAilment => {
            println!("How many nulls does it add?");
            ans = read_num();
            if ans <= 0 {
                panic!("Please enter a number greater than 0");
            }
        }
        Attribute::BuffTurns | Attribute::DebuffTurns => {
            println!("By how many turns does it lengthen buffs/debuffs? (negative number for decreasing turns");
            ans = read_num();
        }
        _ => {
            println!("By how many percent does it increase/decrease the stat? (negative number for decrease. Leave out the '%' sign)");
            ans = read_num();
        }
    }
    ans
}

// Get assist skill effect attribute, main menu plus 7 submenus
pub fn get_ass_attribute() -> Attribute {
    println!("\nWhat effect does it have/what attribute does it affect?");
    println!("1: Base stats (Strength etc.)");
    println!("2: Resistances");
    println!("3: All Targets or Single Target Damage");
    println!("4: Elemental damage");
    println!("5: Healing (HP or MP)");
    println!("6: Ailments");
    println!("7: Nulls or Buff/Debuff turns");
    println!("8: Misc (Guard Rate, S.A charge etc.");
    let ans = read_num();
    match ans {
        1 => return get_attr_base(),
        2 => return get_attr_res(),
        3 => return get_attr_aoe_st(),
        4 => return get_attr_el(),
        5 => return get_ass_attr_heal(),
        6 => return get_ass_attr_ail(),
        7 => return get_ass_attr_null_turns(),
        8 => return get_ass_attr_misc(),
        _ => panic!("Please enter a number from 1 to 7"),
    }
}

fn get_ass_attr_heal() -> Attribute {
    get_numeric_option(
        "How does it affect healing/what sort of healing does it do?",
        vec![
            Attribute::Heal,
            Attribute::HPRegen,
            Attribute::MPRegen,
            Attribute::HealRate,
        ],
        1,
    )
}

fn get_ass_attr_ail() -> Attribute {
    get_numeric_option(
        "What ailment does it inflict?",
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

fn get_ass_attr_null_turns() -> Attribute {
    get_numeric_option(
        "What does it affect?",
        vec![
            Attribute::NullPhysical,
            Attribute::NullMagical,
            Attribute::NullAilment,
            Attribute::BuffTurns,
            Attribute::DebuffTurns,
        ],
        1,
    )
}

fn get_ass_attr_misc() -> Attribute {
    get_numeric_option(
        "What does it affect?",
        vec![
            Attribute::GuardRate,
            Attribute::CounterRate,
            Attribute::CriticalRate,
            Attribute::PenetrationRate,
            Attribute::SACharge,
        ],
        1,
    )
}
