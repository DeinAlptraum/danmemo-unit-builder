pub trait HumanReadable {
    fn to_str(&self) -> String;
}

impl<T: HumanReadable> HumanReadable for Option<T> {
    fn to_str(&self) -> String {
        match self {
            Option::None => "None".to_string(),
            Option::Some(x) => x.to_str(),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum UnitType {
    Adventurer,
    Assist,
}

impl HumanReadable for UnitType {
    fn to_str(&self) -> String {
        match self {
            UnitType::Adventurer => "Adventurer",
            UnitType::Assist => "Assist",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Target {
    Foes,
    Foe,
    Allies,
    Auto,
}

impl HumanReadable for Target {
    fn to_str(&self) -> String {
        match self {
            Target::Foes => "Foes",
            Target::Foe => "Foe",
            Target::Allies => "Allies",
            Target::Auto => "Self",
        }
        .to_string()
    }
}

impl Target {
    pub fn to_json(&self) -> &str {
        match self {
            Target::Foes => "foes",
            Target::Foe => "foe",
            Target::Allies => "allies",
            Target::Auto => "self",
        }
    }
}

// Assist related enums
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Attribute {
    // Buffs & Debuffs
    // Base Stats
    Strength,
    Endurance,
    Dexterity,
    Agility,
    Magic,

    // Resistances
    PhysicalResistance,
    MagicResistance,
    AoEResistance,
    STResistance,

    // Ailment resists
    SleepResist,
    StunResist,
    SealResist,
    SlowResist,
    TauntResist,
    PoisonResist,
    AilmentResist,

    // Elementals
    LightResistance,
    DarkResistance,
    FireResistance,
    WaterResistance,
    ThunderResistance,
    EarthResistance,
    WindResistance,
    AllElementsResistance,

    LightDamage,
    DarkDamage,
    FireDamage,
    WaterDamage,
    ThunderDamage,
    EarthDamage,
    WindDamage,

    // Auxiliary RNG Buffs
    GuardRate,
    CounterRate,
    CriticalRate,
    PenetrationRate,

    // Others
    SACharge,

    // Non-Buff/Debuff Effects
    // Heals
    Heal,
    HPRegen,
    MPHeal,
    MPRegen,
    HealRate,

    // Ailments
    Sleep,
    Stun,
    Seal,
    Slow,
    Taunt,
    Poison,

    // Nulls
    NullPhysical,
    NullMagical,
    NullAilment,

    // (De)buff turns
    BuffTurns,
    DebuffTurns,

    // Combined Attributes
    StrengthAndMagic,
}

impl HumanReadable for Attribute {
    fn to_str(&self) -> String {
        match self {
            Attribute::Strength => "Str.",
            Attribute::Endurance => "End.",
            Attribute::Dexterity => "Dex.",
            Attribute::Agility => "Agi.",
            Attribute::Magic => "Mag.",
            Attribute::PhysicalResistance => "P.Resist",
            Attribute::MagicResistance => "M.Resist",
            Attribute::AoEResistance => "Dmg. received (Attack Type: All Targets)",
            Attribute::STResistance => "Dmg. received (Attack Type: Single Target)",
            Attribute::LightResistance => "Light Resist",
            Attribute::DarkResistance => "Dark Resist",
            Attribute::FireResistance => "Fire Resist",
            Attribute::WaterResistance => "Water Resist",
            Attribute::ThunderResistance => "Thunder Resist",
            Attribute::EarthResistance => "Earth Resist",
            Attribute::WindResistance => "Wind Resist",
            Attribute::AllElementsResistance => "Resistance against all Elements",
            Attribute::LightDamage => "Light Attack Dmg.",
            Attribute::DarkDamage => "Dark Attack Dmg.",
            Attribute::FireDamage => "Fire Attack Dmg.",
            Attribute::WaterDamage => "Water Attack Dmg.",
            Attribute::ThunderDamage => "Thunder Attack Dmg.",
            Attribute::EarthDamage => "Earth Attack Dmg.",
            Attribute::WindDamage => "Wind Attack Dmg.",
            Attribute::GuardRate => "Guard Rate",
            Attribute::CounterRate => "Counter Rate",
            Attribute::CriticalRate => "Critical Rate",
            Attribute::PenetrationRate => "Penetration Rate",
            Attribute::Heal => "HP Heal",
            Attribute::HPRegen => "HP Regen/turn",
            Attribute::MPHeal => "MP Heal",
            Attribute::MPRegen => "MP Regen/turn",
            Attribute::HealRate => "Heal",
            Attribute::Sleep => "Sleep",
            Attribute::Stun => "Stun",
            Attribute::Seal => "Seal",
            Attribute::Slow => "Slow",
            Attribute::Taunt => "Taunt",
            Attribute::Poison => "Poison",
            Attribute::SleepResist => "Sleep Resist",
            Attribute::StunResist => "Stun Resist",
            Attribute::SealResist => "Seal Resist",
            Attribute::SlowResist => "Slow Resist",
            Attribute::TauntResist => "Taunt Resist",
            Attribute::PoisonResist => "Poison Resist",
            Attribute::AilmentResist => "Ailment Resist",
            Attribute::NullPhysical => "Null P.Attack",
            Attribute::NullMagical => "Null M.Attack",
            Attribute::NullAilment => "Null Ailments",
            Attribute::BuffTurns => "Status Buff",
            Attribute::DebuffTurns => "Status Debuff",
            Attribute::SACharge => "S.A Gauge Charge gain",
            Attribute::StrengthAndMagic => "Str. and Mag.",
        }
        .to_string()
    }
}

impl Attribute {
    pub fn to_json(&self) -> &str {
        match self {
            Attribute::Strength => "strength",
            Attribute::Endurance => "endurance",
            Attribute::Dexterity => "dexterity",
            Attribute::Agility => "agility",
            Attribute::Magic => "magic",
            Attribute::PhysicalResistance => "physical_resist",
            Attribute::MagicResistance => "magic_resist",
            Attribute::AoEResistance => "all_damage_resist",
            Attribute::STResistance => "single_damage_resist",
            Attribute::SleepResist => "sleep_resist",
            Attribute::StunResist => "stun_resist",
            Attribute::SealResist => "seal_resist",
            Attribute::SlowResist => "slow_resist",
            Attribute::TauntResist => "taunt_resist",
            Attribute::PoisonResist => "poison_resist",
            Attribute::AilmentResist => "ailment_resist",
            Attribute::LightResistance => "light_resist",
            Attribute::DarkResistance => "dark_resist",
            Attribute::FireResistance => "fire_resist",
            Attribute::WaterResistance => "water_resist",
            Attribute::ThunderResistance => "thunder_resist",
            Attribute::EarthResistance => "earth_resist",
            Attribute::WindResistance => "wind_resist",
            Attribute::AllElementsResistance => "fire_water_thunder_earth_wind_light_dark_resist",
            Attribute::LightDamage => "light_attack",
            Attribute::DarkDamage => "dark_attack",
            Attribute::FireDamage => "fire_attack",
            Attribute::WaterDamage => "water_attack",
            Attribute::ThunderDamage => "thunder_attack",
            Attribute::EarthDamage => "earth_attack",
            Attribute::WindDamage => "wind_attack",
            Attribute::GuardRate => "guard_rate",
            Attribute::CounterRate => "counter_rate",
            Attribute::CriticalRate => "critical_rate",
            Attribute::PenetrationRate => "penetration_rate",
            Attribute::SACharge => "sa_gauge_charge",
            Attribute::Heal => "heal",
            Attribute::HPRegen => "hp_regen",
            Attribute::MPHeal => "mp_heal",
            Attribute::MPRegen => "mp_regen",
            Attribute::HealRate => "heal",
            Attribute::Sleep => "sleep",
            Attribute::Stun => "stun",
            Attribute::Seal => "seal",
            Attribute::Slow => "slow",
            Attribute::Taunt => "taunt",
            Attribute::Poison => "poison",
            Attribute::NullPhysical => "null_physical_attack_no_special",
            Attribute::NullMagical => "null_magic_attack_no_special",
            Attribute::NullAilment => "null_ailments",
            Attribute::BuffTurns => "status_buff",
            Attribute::DebuffTurns => "status_debuff",
            Attribute::StrengthAndMagic => "strength_magic",
        }
    }
}

// Adventurer related enums
#[derive(PartialEq, Eq, Clone)]
pub enum Element {
    None,
    Light,
    Dark,
    Fire,
    Water,
    Thunder,
    Earth,
    Wind,
}

impl HumanReadable for Element {
    fn to_str(&self) -> String {
        match self {
            Element::None => "None",
            Element::Light => "Light",
            Element::Dark => "Dark",
            Element::Fire => "Fire",
            Element::Water => "Water",
            Element::Thunder => "Thunder",
            Element::Earth => "Earth",
            Element::Wind => "Wind",
        }
        .to_string()
    }
}

impl Element {
    pub fn to_json(&self) -> &str {
        match self {
            Element::None => "None",
            Element::Light => "light",
            Element::Dark => "dark",
            Element::Fire => "fire",
            Element::Water => "water",
            Element::Thunder => "thunder",
            Element::Earth => "earth",
            Element::Wind => "wind",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum AdventurerType {
    Physical,
    Magic,
    Balance,
}

impl HumanReadable for AdventurerType {
    fn to_str(&self) -> String {
        match self {
            AdventurerType::Physical => "Physical",
            AdventurerType::Magic => "Magic",
            AdventurerType::Balance => "Balance",
        }
        .to_string()
    }
}

impl AdventurerType {
    pub fn to_json(&self) -> &str {
        match self {
            AdventurerType::Physical => "physical_type",
            AdventurerType::Magic => "magic_type",
            AdventurerType::Balance => "balance_type",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum DamageType {
    Physical,
    Magic,
}

impl HumanReadable for DamageType {
    fn to_str(&self) -> String {
        match self {
            DamageType::Physical => "Physical Damage",
            DamageType::Magic => "Magic Damage",
        }
        .to_string()
    }
}

impl DamageType {
    pub fn to_json(&self) -> &str {
        match self {
            DamageType::Physical => "physical_attack",
            DamageType::Magic => "magic_attack",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum SkillModifier {
    Low,
    Medium,
    High,
    Super,
    Ultra,
}

impl HumanReadable for SkillModifier {
    fn to_str(&self) -> String {
        match self {
            SkillModifier::Low => "Low",
            SkillModifier::Medium => "Mid",
            SkillModifier::High => "Hi",
            SkillModifier::Super => "Super",
            SkillModifier::Ultra => "Ultra",
        }
        .to_string()
    }
}

impl SkillModifier {
    pub fn to_json(&self) -> &str {
        match self {
            SkillModifier::Low => "low",
            SkillModifier::Medium => "medium",
            SkillModifier::High => "high",
            SkillModifier::Super => "super",
            SkillModifier::Ultra => "ultra",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum Speed {
    Slow,
    None,
    Fast,
}

impl HumanReadable for Speed {
    fn to_str(&self) -> String {
        match self {
            Speed::Slow => "Slow",
            Speed::None => "None",
            Speed::Fast => "Fast",
        }
        .to_string()
    }
}

impl Speed {
    pub fn to_json(&self) -> &str {
        match self {
            Speed::Slow => "slow",
            Speed::None => "None",
            Speed::Fast => "fast",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum TempBoost {
    Boost,
    GreatBoost,
}

impl HumanReadable for TempBoost {
    fn to_str(&self) -> String {
        match self {
            TempBoost::Boost => "Temp. Boost",
            TempBoost::GreatBoost => "Great Temp. Boost",
        }
        .to_string()
    }
}

impl TempBoost {
    pub fn to_json(&self, dt: &DamageType) -> &str {
        match (self, dt) {
            (TempBoost::Boost, DamageType::Magic) => "mag_temp",
            (TempBoost::Boost, DamageType::Physical) => "str_temp",
            (TempBoost::GreatBoost, DamageType::Magic) => "great_mag_temp",
            (TempBoost::GreatBoost, DamageType::Physical) => "great_str_temp",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum RateAttribute {
    Unguard,
    Uncounter,
    Critical,
    Penetration,
}

impl HumanReadable for RateAttribute {
    fn to_str(&self) -> String {
        match self {
            RateAttribute::Unguard => "Unguard Rate",
            RateAttribute::Uncounter => "Uncounter Rate",
            RateAttribute::Critical => "Critical Rate",
            RateAttribute::Penetration => "Penetration Rate",
        }
        .to_string()
    }
}

impl RateAttribute {
    pub fn to_json(&self) -> &str {
        match self {
            RateAttribute::Unguard => "unguard_rate",
            RateAttribute::Uncounter => "uncounter_rte",
            RateAttribute::Critical => "critical_rate",
            RateAttribute::Penetration => "penetration_rate",
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BuffType {
    Buff,
    Debuff,
}

impl HumanReadable for BuffType {
    fn to_str(&self) -> String {
        match self {
            BuffType::Buff => "Buff",
            BuffType::Debuff => "Debuff",
        }
        .to_string()
    }
}

impl BuffType {
    pub fn to_json(&self) -> String {
        match self {
            BuffType::Buff => "buff",
            BuffType::Debuff => "debuff",
        }
        .to_string()
    }

    pub fn to_json_long(&self) -> String {
        match self {
            BuffType::Buff => "status_buff",
            BuffType::Debuff => "status_debuff",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq)]
pub enum HealModifier {
    Numeric(u32),
    SkillModifier(SkillModifier),
}

impl HealModifier {
    pub fn to_json(&self) -> String {
        match self {
            HealModifier::Numeric(x) => format!("{}", x),
            HealModifier::SkillModifier(x) => x.to_json().to_string(),
        }
    }
}
