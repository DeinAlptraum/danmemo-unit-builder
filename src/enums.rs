#[derive(PartialEq, Eq)]
pub enum UnitType {
    Adventurer,
    Assist,
}

#[derive(PartialEq, Eq)]
pub enum Element {
    Light,
    Dark,
    Fire,
    Water,
    Thunder,
    Earth,
    Wind,
}

impl Element {
    pub fn to_json(&self) -> &str {
        match self {
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

#[derive(PartialEq, Eq)]
pub enum AdventurerType {
    Physical,
    Magic,
    Balance,
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
pub enum Target {
    Foes,
    //Foe,
    Allies,
    Auto,
}

impl Target {
    pub fn to_json(&self) -> &str {
        match self {
            Target::Foes => "foes",
            //Target::Foe => "foe",
            Target::Allies => "allies",
            Target::Auto => "self",
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Target::Foes => "Foes",
            //Target::Foe => "Foe",
            Target::Allies => "Allies",
            Target::Auto => "Self",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
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

    // Elementals
    LightResistance,
    DarkResistance,
    FireResistance,
    WaterResistance,
    ThunderResistance,
    EarthResistance,
    WindResistance,

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
            Attribute::LightResistance => "light_resist",
            Attribute::DarkResistance => "dark_resist",
            Attribute::FireResistance => "fire_resist",
            Attribute::WaterResistance => "water_resist",
            Attribute::ThunderResistance => "thunder_resist",
            Attribute::EarthResistance => "earth_resist",
            Attribute::WindResistance => "wind_resist",
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

            Attribute::Heal => "HP Heal",
            Attribute::HPRegen => "hp_regen",
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
        }
    }

    pub fn to_str(&self) -> &str {
        let res = match self {
            Attribute::Strength => "Str.",
            Attribute::Endurance => "End.",
            Attribute::Dexterity => "Dex.",
            Attribute::Agility => "Agi.",
            Attribute::Magic => "Mag.",
            Attribute::PhysicalResistance => "P.Resist",
            Attribute::MagicResistance => "M.Resist",
            Attribute::AoEResistance => "Dmg. received (Attack Type: All Targets)",
            Attribute::STResistance => "Dmg. received (Attack Type: Single Target",
            Attribute::LightResistance => "Light Resist",
            Attribute::DarkResistance => "Dark Resist",
            Attribute::FireResistance => "Fire Resist",
            Attribute::WaterResistance => "Water Resist",
            Attribute::ThunderResistance => "Thunder Resist",
            Attribute::EarthResistance => "Earth Resist",
            Attribute::WindResistance => "Wind Resist",
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
            Attribute::NullPhysical => "Null P.Attack",
            Attribute::NullMagical => "Null M.Attack",
            Attribute::NullAilment => "Null Ailments",
            Attribute::BuffTurns => "Status Buff",
            Attribute::DebuffTurns => "Status Debuff",
            Attribute::SACharge => "S.A Gauge Charge gain",
        };
        res
    }
}
