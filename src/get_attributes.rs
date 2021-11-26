use crate::enums::*;
use crate::HumanReadable;
use std::convert::TryInto;
use std::io::{stdin, stdout, Write};

// Helper functions
pub fn read_num() -> i32 {
    let mut input = String::new();

    loop {
        stdin().read_line(&mut input).expect("Error reading input. I have no idea why this is happening, but please tell me anyway so I can try to fix it.");
        match input.trim().parse() {
            Ok(result) => return result,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        }
    }
}

pub fn read_str() -> String {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error!");
        if input == "" {
            println!("Please enter something");
            continue;
        }
        return input.trim().to_string();
    }
}

pub fn read_multistring() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error!");
    input.trim().to_string();

    let mut res = Vec::new();
    for entry in input.split(" ") {
        res.push(entry.trim().to_string());
    }

    res
}

pub fn read_multinum() -> Vec<i32> {
    'outer: loop {
        let strs = read_multistring();
        let mut nums = Vec::new();
        for entry in &strs {
            if entry != "" {
                match entry.parse() {
                    Ok(x) => nums.push(x),
                    Err(_) => {
                        println!("Please enter only numbers");
                        continue 'outer;
                    }
                }
            }
        }
        return nums;
    }
}

pub fn get_numeric_option<T: HumanReadable + std::cmp::Eq + std::clone::Clone>(
    question: &str,
    options: Vec<T>,
    start: u32,
) -> T {
    println!("{}", question);
    let mut counter = start;
    for op in &options {
        print!("{}: {}", counter, op.to_str());
        if op != options.last().unwrap() {
            if (counter - start + 1) % 5 == 0 {
                println!();
            } else {
                print!(", ");
            }
        }
        counter += 1;
    }
    println!();
    stdout().flush().unwrap();

    loop {
        let ans = read_num() - (start as i32);
        if ans < (options.len() as i32) {
            return options[ans as usize].clone();
        } else {
            let end = start + (options.len() as u32) - 1;
            println!("Please enter a number from {} to {}", start, end);
            continue;
        }
    }
}

pub fn get_u32(question: &str) -> u32 {
    println!("{}", question);

    loop {
        let ans = read_num();
        if ans <= 0 {
            println!("Please enter a number greater than 0");
            continue;
        }
        return ans.try_into().unwrap();
    }
}

pub fn get_chance(question: &str) -> u32 {
    loop {
        let res = get_u32(question);
        if res > 100 {
            println!("Please enter a number no greater than 100");
            continue;
        }
        return res;
    }
}

// getters
// Header
pub fn get_unit_type() -> UnitType {
    get_numeric_option(
        "Adventurer or Assist?",
        vec![UnitType::Adventurer, UnitType::Assist],
        1,
    )
}

pub fn get_title() -> String {
    println!(
        "\nWhat is the unit's title? (e.g. \"Decisive Will\" in \"[Decisive Will] Bell Cranel\")"
    );
    let title = read_str();
    title
}

pub fn get_name() -> String {
    println!(
        "\nWhat is the character's name? (e.g. \"Bell Cranel\" in \"[Decisive Will] Bell Cranel\")"
    );
    let name = read_str();
    name
}

pub fn get_stars() -> i32 {
    println!("\nHow many stars does the unit have? (1-4)");

    loop {
        let stars = read_num();
        if stars > 4 || stars < 1 {
            println!("Please enter a number from 1 to 4");
            continue;
        }
        return stars;
    }
}

pub fn get_limited() -> bool {
    println!("\nIs the unit time-limited? (n/no, y/yes)");

    loop {
        let limited = read_str();
        match limited.as_str() {
            "n" => return false,
            "no" => return false,
            "yes" => return true,
            "y" => return true,
            _ => {
                println!("Please enter 'n', 'no', 'y' or 'yes'");
                continue;
            }
        }
    }
}

// Stats
pub fn get_one_lb_stats(i: i32) -> Vec<i32> {
    println!(
        "\nPlease enter the unit's stats at +{} separated by spaces in the following order:",
        i
    );
    println!("HP MP Str. End. Dex. Agi. Mag.");

    loop {
        let stats = read_multinum();
        if stats.len() != 7 {
            println!("Please enter exactly 7 numbers");
            continue;
        }
        return stats;
    }
}

// Adventurer & Assist effect shared getters
pub fn get_attr_base() -> Attribute {
    get_numeric_option(
        "\nWhich attribute does it affect?",
        vec![
            Attribute::Strength,
            Attribute::Magic,
            Attribute::Agility,
            Attribute::Dexterity,
            Attribute::Endurance,
        ],
        1,
    )
}

pub fn get_attr_res() -> Attribute {
    get_numeric_option(
        "\nWhich resistance does it affect?",
        vec![
            Attribute::PhysicalResistance,
            Attribute::MagicResistance,
            Attribute::LightResistance,
            Attribute::DarkResistance,
            Attribute::FireResistance,
            Attribute::WaterResistance,
            Attribute::ThunderResistance,
            Attribute::EarthResistance,
            Attribute::WindResistance,
            Attribute::AllElementsResistance,
            Attribute::SleepResist,
            Attribute::StunResist,
            Attribute::SealResist,
            Attribute::SlowResist,
            Attribute::TauntResist,
            Attribute::PoisonResist,
            Attribute::AilmentResist,
        ],
        1,
    )
}

pub fn get_attr_res_no_ailment() -> Attribute {
    get_numeric_option(
        "\nWhich resistance does it affect?",
        vec![
            Attribute::PhysicalResistance,
            Attribute::MagicResistance,
            Attribute::LightResistance,
            Attribute::DarkResistance,
            Attribute::FireResistance,
            Attribute::WaterResistance,
            Attribute::ThunderResistance,
            Attribute::EarthResistance,
            Attribute::WindResistance,
            Attribute::AllElementsResistance,
        ],
        1,
    )
}

pub fn get_attr_aoe_st() -> Attribute {
    get_numeric_option(
        "\nWhat type of damage does it affect?",
        vec![Attribute::AoEResistance, Attribute::STResistance],
        1,
    )
}

pub fn get_attr_el() -> Attribute {
    get_numeric_option(
        "\nWhich element's damage does it affect?",
        vec![
            Attribute::LightDamage,
            Attribute::DarkDamage,
            Attribute::FireDamage,
            Attribute::WaterDamage,
            Attribute::ThunderDamage,
            Attribute::EarthDamage,
            Attribute::WindDamage,
        ],
        1,
    )
}
