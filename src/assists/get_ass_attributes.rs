use crate::enums::*;
use crate::get_attributes::*;

// Assist Skill
pub fn get_ass_skill_name() -> String {
    println!("\nWhat is the name of the assist's skill? (without '+' at the end)");
    let name = read_nonempty_str();
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
        | Attribute::Poison
        | Attribute::Charm => Target::Foes,
        _ => get_ass_target_no_ailment(),
    }
}

fn get_ass_target_no_ailment() -> Target {
    println!("\nWho is the effect's target? (1: Self, 2: Allies, 3: Foes)");

    loop {
        let input = read_num();
        match input {
            1 => return Target::Auto,
            2 => return Target::Allies,
            3 => return Target::Foes,
            _ => {
                println!("Please enter a number from 1 to 3");
                continue;
            }
        }
    }
}

pub fn get_ass_modifier(attr: &Attribute) -> i32 {
    match attr {
        Attribute::AoEResistance | Attribute::STResistance => get_ass_modifier_aoe_st(),
        Attribute::MPRegen => get_ass_modifier_mp_regen(),
        Attribute::Sleep
        | Attribute::Stun
        | Attribute::Seal
        | Attribute::Slow
        | Attribute::Taunt
        | Attribute::Poison
        | Attribute::Charm => get_ass_modifier_ailment(),
        Attribute::NullPhysical | Attribute::NullMagical | Attribute::NullAilment => {
            get_ass_modifier_null()
        }
        Attribute::BuffTurns | Attribute::DebuffTurns => get_ass_modifier_buff_turns(),
        _ => get_ass_modifier_generic(),
    }
}

fn get_ass_modifier_aoe_st() -> i32 {
    println!("\nBy how many percent does it increase or decrease damage? (leave out the '%' sign)");
    -1 * read_num()
}

fn get_ass_modifier_mp_regen() -> i32 {
    println!("\nHow many MP does it regen per turn?");

    loop {
        let ans = read_num();
        if ans <= 0 {
            println!("Please enter a number greater than 0");
            continue;
        }
        return ans;
    }
}

fn get_ass_modifier_ailment() -> i32 {
    println!("\nWhat is the chance to activate the ailment? (leave out the '%' sign)");

    loop {
        let ans = read_num();
        if ans <= 0 {
            println!("Please enter a number greater than 0");
            continue;
        }
        return ans;
    }
}

fn get_ass_modifier_null() -> i32 {
    println!("\nHow many nulls does it add?");

    loop {
        let ans = read_num();
        if ans <= 0 {
            println!("Please enter a number greater than 0");
            continue;
        }
        return ans;
    }
}

fn get_ass_modifier_buff_turns() -> i32 {
    println!(
        "\nBy how many turns does it lengthen buffs/debuffs? (negative number for decreasing turns)"
    );
    read_num()
}

fn get_ass_modifier_generic() -> i32 {
    println!("\nBy how many percent does it increase/decrease the stat? (negative number for decrease. Leave out the '%' sign)");
    read_num()
}

// Get assist skill effect attribute, main menu plus 7 submenus
pub fn get_ass_attribute() -> Attribute {
    println!("\nWhat effect does the regular skill have/what attribute does it affect?");
    println!("1: Base stats (Strength etc.)");
    println!("2: Resistances");
    println!("3: All Targets or Single Target Damage");
    println!("4: Elemental damage");
    println!("5: Healing (HP or MP)");
    println!("6: Ailments");
    println!("7: Nulls or Buff/Debuff turns");
    println!("8: Misc (Guard Rate, S.A charge etc.)");

    loop {
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
            _ => {
                println!("Please enter a number from 1 to 7");
                continue;
            }
        }
    }
}

fn get_ass_attr_heal() -> Attribute {
    get_numeric_option(
        "\nHow does it affect healing/what sort of healing does it do?",
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
        "\nWhat ailment does it inflict?",
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

fn get_ass_attr_null_turns() -> Attribute {
    get_numeric_option(
        "\nWhat does it affect?",
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
        "\nWhat does it affect?",
        vec![
            Attribute::CriticalRate,
            Attribute::PenetrationRate,
            Attribute::CounterRate,
            Attribute::GuardRate,
            Attribute::SACharge,
        ],
        1,
    )
}

// Instant Effects
pub fn get_ia_base_duration() -> u32 {
    get_u32("\nHow many times can the effect be activated at base?")
}

pub fn get_ia_mlb_duration() -> u32 {
    get_u32("How many times can the effect be activated at MLB?")
}

pub fn get_ia_max_activations() -> u32 {
    get_u32("\nHow many times per turn can the effect be activated?")
}

pub fn get_instant_damage_type() -> DamageType {
    get_numeric_option(
        "\nWhat type of damage does the attack do?",
        vec![DamageType::Physical, DamageType::Magic],
        1,
    )
}

pub fn get_instant_element() -> Element {
    get_numeric_option(
        "\nWhat is the attack's element?",
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
