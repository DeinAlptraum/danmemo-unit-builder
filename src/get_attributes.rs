use crate::enums::*;
use crate::HumanReadable;
use std::convert::TryInto;
use std::io::*;

// Helper functions
pub fn read_num() -> i32 {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Error!");
    match input.trim().parse() {
        Ok(result) => result,
        Err(_) => panic!("Error reading input"),
    }
}

pub fn read_str() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error!");
    input.trim().to_string()
}

pub fn read_multinum() -> Vec<i32> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error!");
    input.trim().to_string();

    let mut res = Vec::new();
    for entry in input.split(" ") {
        match entry.parse() {
            Ok(x) => res.push(x),
            Err(_) => panic!("Error reading input"),
        }
    }

    res
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
            if counter > start && (counter - start) % 5 == 0 {
                println!();
            } else {
                print!(", ");
            }
        }
        counter += 1;
    }
    println!();

    stdout().flush().unwrap();
    let ans = read_num() - (start as i32);
    if ans < (options.len() as i32) {
        return options[ans as usize].clone();
    } else {
        let end = start + (options.len() as u32) - 1;
        panic!("Please enter a number from {} to {}", start, end);
    }
}

pub fn get_u32(question: &str) -> u32 {
    println!("{}", question);
    let ans = read_num();
    if ans <= 0 {
        panic!("Please enter a number greater than 0");
    }
    ans.try_into().unwrap()
}

pub fn get_chance(question: &str) -> u32 {
    let res = get_u32(question);
    if res > 100 {
        panic!("Please enter a number no greater than 100");
    }
    res
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
    match limited.as_str() {
        "n" => false,
        "no" => false,
        "yes" => true,
        "y" => true,
        _ => panic!("Please enter 'n, 'no', 'y' or 'yes'"),
    }
}

// Stats
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

// Adventurer & Assist effect shared getters
pub fn get_attr_base() -> Attribute {
    get_numeric_option(
        "Which attribute does it affect?",
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
        "Which resistance does it affect?",
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
        "Which resistance does it affect?",
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
        "What type of damage does it affect?",
        vec![Attribute::AoEResistance, Attribute::STResistance],
        1,
    )
}

pub fn get_attr_el() -> Attribute {
    get_numeric_option(
        "Which element's damage does it affect?",
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
