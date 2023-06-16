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
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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
    CharmResist,
    AilmentResist,

    // Elementals
    FireResistance,
    WaterResistance,
    ThunderResistance,
    EarthResistance,
    WindResistance,
    LightResistance,
    DarkResistance,

    FireDamage,
    WaterDamage,
    ThunderDamage,
    EarthDamage,
    WindDamage,
    LightDamage,
    DarkDamage,

    // Auxiliary RNG Buffs
    CriticalRate,
    PenetrationRate,
    CounterRate,
    GuardRate,

    CriticalDamage,
    PenetrationDamage,
    CounterDamage,

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
    Charm,

    // Nulls
    NullPhysical,
    NullMagical,
    NullAilment,

    // (De)buff turns
    BuffTurns,
    DebuffTurns,
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
            Attribute::FireResistance => "Fire Resist",
            Attribute::WaterResistance => "Water Resist",
            Attribute::ThunderResistance => "Thunder Resist",
            Attribute::EarthResistance => "Earth Resist",
            Attribute::WindResistance => "Wind Resist",
            Attribute::LightResistance => "Light Resist",
            Attribute::DarkResistance => "Dark Resist",
            Attribute::FireDamage => "Fire Attack Dmg.",
            Attribute::WaterDamage => "Water Attack Dmg.",
            Attribute::ThunderDamage => "Thunder Attack Dmg.",
            Attribute::EarthDamage => "Earth Attack Dmg.",
            Attribute::WindDamage => "Wind Attack Dmg.",
            Attribute::LightDamage => "Light Attack Dmg.",
            Attribute::DarkDamage => "Dark Attack Dmg.",
            Attribute::CriticalRate => "Critical Rate",
            Attribute::PenetrationRate => "Penetration Rate",
            Attribute::CounterRate => "Counter Rate",
            Attribute::GuardRate => "Guard Rate",
            Attribute::CriticalDamage => "Dmg. Upon Critical",
            Attribute::PenetrationDamage => "Dmg. Upon Penetration",
            Attribute::CounterDamage => "Counter Damage",
            Attribute::Heal => "HP Heal",
            Attribute::HPRegen => "HP Regen turn",
            Attribute::MPHeal => "MP Heal",
            Attribute::MPRegen => "MP Regen turn",
            Attribute::HealRate => "Heal",
            Attribute::Sleep => "Sleep",
            Attribute::Stun => "Stun",
            Attribute::Seal => "Seal",
            Attribute::Slow => "Slow",
            Attribute::Taunt => "Taunt",
            Attribute::Poison => "Poison",
            Attribute::Charm => "Charm",
            Attribute::SleepResist => "Sleep Resist",
            Attribute::StunResist => "Stun Resist",
            Attribute::SealResist => "Seal Resist",
            Attribute::SlowResist => "Slow Resist",
            Attribute::TauntResist => "Taunt Resist",
            Attribute::PoisonResist => "Poison Resist",
            Attribute::CharmResist => "Charm Resist",
            Attribute::AilmentResist => "Ailment Resist",
            Attribute::NullPhysical => "Null P.Attack",
            Attribute::NullMagical => "Null M.Attack",
            Attribute::NullAilment => "Null Ailments",
            Attribute::BuffTurns => "Status Buff",
            Attribute::DebuffTurns => "Status Debuff",
            Attribute::SACharge => "S.A Gauge Charge gain",
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
            Attribute::CharmResist => "charm_resist",
            Attribute::AilmentResist => "ailment_resist",
            Attribute::FireResistance => "fire_resist",
            Attribute::WaterResistance => "water_resist",
            Attribute::ThunderResistance => "thunder_resist",
            Attribute::EarthResistance => "earth_resist",
            Attribute::WindResistance => "wind_resist",
            Attribute::LightResistance => "light_resist",
            Attribute::DarkResistance => "dark_resist",
            Attribute::FireDamage => "fire_attack",
            Attribute::WaterDamage => "water_attack",
            Attribute::ThunderDamage => "thunder_attack",
            Attribute::EarthDamage => "earth_attack",
            Attribute::WindDamage => "wind_attack",
            Attribute::LightDamage => "light_attack",
            Attribute::DarkDamage => "dark_attack",
            Attribute::CriticalRate => "critical_rate",
            Attribute::PenetrationRate => "penetration_rate",
            Attribute::CounterRate => "counter_rate",
            Attribute::GuardRate => "guard_rate",
            Attribute::CriticalDamage => "critical_damage",
            Attribute::PenetrationDamage => "penetration_damage",
            Attribute::CounterDamage => "counter_damage",
            Attribute::SACharge => "sa_gauge_charge",
            Attribute::Heal => "heal",
            Attribute::HPRegen => "hp_regen",
            Attribute::MPHeal => "mp_heal",
            Attribute::MPRegen => "mp_regen",
            Attribute::HealRate => "heal_modifier",
            Attribute::Sleep => "sleep",
            Attribute::Stun => "stun",
            Attribute::Seal => "seal",
            Attribute::Slow => "slow",
            Attribute::Taunt => "taunt",
            Attribute::Poison => "poison",
            Attribute::Charm => "charm",
            Attribute::NullPhysical => "null_physical_attack_no_special",
            Attribute::NullMagical => "null_magic_attack_no_special",
            Attribute::NullAilment => "null_ailment",
            Attribute::BuffTurns => "status_buff",
            Attribute::DebuffTurns => "status_debuff",
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum LimitedType {
    Regular,
    TimeLimited,
    HeroFesta,
}

impl HumanReadable for LimitedType {
    fn to_str(&self) -> String {
        match self {
            LimitedType::Regular => "Regular",
            LimitedType::TimeLimited => "Time-Limited",
            LimitedType::HeroFesta => "Hero Festa",
        }
        .to_string()
    }
}

impl LimitedType {
    pub fn to_json(&self) -> &str {
        match self {
            LimitedType::Regular => "0",
            LimitedType::TimeLimited => "1",
            LimitedType::HeroFesta => "2",
        }
    }
}

// Adventurer related enums
#[derive(PartialEq, Eq, Clone)]
pub enum Element {
    None,
    Fire,
    Water,
    Thunder,
    Earth,
    Wind,
    Light,
    Dark,
}

impl HumanReadable for Element {
    fn to_str(&self) -> String {
        match self {
            Element::None => "None",
            Element::Fire => "Fire",
            Element::Water => "Water",
            Element::Thunder => "Thunder",
            Element::Earth => "Earth",
            Element::Wind => "Wind",
            Element::Light => "Light",
            Element::Dark => "Dark",
        }
        .to_string()
    }
}

impl Element {
    pub fn to_json(&self) -> &str {
        match self {
            Element::None => "None",
            Element::Fire => "fire",
            Element::Water => "water",
            Element::Thunder => "thunder",
            Element::Earth => "earth",
            Element::Wind => "wind",
            Element::Light => "light",
            Element::Dark => "dark",
        }
    }

    fn str_to_type(inp: &str) -> Option<Element> {
        let inp = inp.trim().to_lowercase();
        match inp.as_str() {
            "fire" => Some(Element::Fire),
            "water" => Some(Element::Water),
            "thunder" => Some(Element::Thunder),
            "earth" => Some(Element::Earth),
            "wind" => Some(Element::Wind),
            "light" => Some(Element::Light),
            "dark" => Some(Element::Dark),
            _ => None,
        }
    }

    pub fn effective_against(&self) -> Element {
        match self {
            Element::None => Element::None,
            Element::Fire => Element::Wind,
            Element::Water => Element::Fire,
            Element::Thunder => Element::Water,
            Element::Earth => Element::Thunder,
            Element::Wind => Element::Earth,
            Element::Light => Element::Dark,
            Element::Dark => Element::Light,
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

    pub fn to_short_str(&self) -> String {
        match self {
            DamageType::Physical => "P.",
            DamageType::Magic => "M.",
        }
        .to_string()
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
            SkillModifier::Low => "Lo",
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
            TempBoost::Boost => "temp Boost",
            TempBoost::GreatBoost => "temp Great Boost",
        }
        .to_string()
    }
}

impl TempBoost {
    pub fn to_json(&self, dt: &DamageType) -> &str {
        match (self, dt) {
            (TempBoost::Boost, DamageType::Magic) => "normal2_mag",
            (TempBoost::Boost, DamageType::Physical) => "normal2_str",
            (TempBoost::GreatBoost, DamageType::Magic) => "great_mag",
            (TempBoost::GreatBoost, DamageType::Physical) => "great_str",
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
            RateAttribute::Uncounter => "uncounter_rate",
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

#[derive(PartialEq, Eq)]
pub enum DevelopmentSkillType {
    Unknown(String),

    // Elemental & Counters
    Manifestation(Element, DamageType, u32), // modifier resist
    WillOf(Element, DamageType),             // elemental counters
    Resistance(Element, u32),                // modifier resist

    // Counters
    Bravery(u32),  // regular attack, heal dmg, water Bravery???
    Encouragement, // Counters Lo heal lowest Ally
    Blessing,      // counters extend buff 1 turn
    Flashback,     // Ultra Crit Rate Counters

    // With base Stats
    // 4 stats
    Hex(u32),         // Mag., End., Agi. & Dex.
    MartialArts(u32), // Str. & End. & Agi. & Dex.
    // 3 stats
    Tattletale(u32),       // Mag. Agi. Dex.
    FightingSpirit(u32),   // Str. Agi. Dex.
    BravePerformance(u32), // Mag. End. Dex.
    BattleBravery(u32),    // Str. End. Dex.
    Rigid(u32),            // End. Agi. Dex.
    // 2 stats
    Forestall(u32),   // Mag. & Agi.
    BattleArts(u32),  // Str. & Dex.
    Concentrate(u32), // Agi. & Dex.
    Instinct(u32),    // Agi. & Dex.
    Climb(u32),       // End. & Agi.
    // 1 stat
    Crush(u32),        // Str.
    FistStrike(u32),   // Str.
    Mage(u32),         // Mag.
    MindsEye(u32),     // Mag.
    Acceleration(u32), // Agi.
    Hunter(u32),       // Agi.
    Crafter(u32),      // Dex.

    // Resistances
    Protection(u32),         // P.Resist & M.Resist
    MagicResistance(u32),    // M.Resist
    StatusResist(u32),       // Ailment Resist
    AbnormalResistance(u32), // Ailment Resist

    // Rate Buffs
    Solid(u32),          // Guard Rate
    Strike(u32),         // crit dmg.
    PiercingStrike(u32), // pen dmg.
    TrueStrike(u32),     // crit & pen dmg.
    CounterAttack(u32),  // Counter-attack, Counter Damage

    Bloom(u32),         // HP & MP Regen
    SpiritHealing(u32), // MP Regen

    // Bell's
    LiarisFreese,
    Luck(u32),

    Killer(EnemyType),
    /*
     * Not included:
     * Unbending: anni 4 water Bell, when countering -1 turn Str/Mag debuffs
     * (Elemental) Ruin: counters reduce foe buff turns by 1
     * (Elemental) Penetration: anni 4 fire Welf & dark Elfy, per-each elemental res debuff dmg bonus
     * (Elemental) Anti Strength: counters remove Str. Buffs
     * Anti Magic: analogously
     * Dark Captor: Love Spell Chigusa, dark M. counters seal
     * Elios Passion: Agi. Buff, all Daphne Units
     * Five Dimension Troia: self mag null on counter, only Cassandra?
     * Earth's Distraction: swimsuit Alicia, counters taunt
     * Fist Strike: Str., only old units
     * Predator: only new Zard, life steal counters debuff Str.
     * Strike L.Sword
     * (something) L.Sword: stat boni when equipped with L.Sword
     * Silence: only Alfia, counters Mag Null Self, remove Foe Mag Buffs
     * Fire Guard: only Vesta, self all elements resist buff and counters AoE heal
     * (Elemental) Pressure: only Epimetheus & Yukina, elemental counters debuff str/mag
     * Dragon Bearer: only Aldo, counters debuff P.Res
     * Magical Arts: buffs Mag. & Dex., only Mariel
     *  Hierophant: heal counters, only Mariel
     * Abyssal Devotee: only Thillelille, ultra crit rate counters
     */
}

impl HumanReadable for DevelopmentSkillType {
    fn to_str(&self) -> String {
        match self {
            DevelopmentSkillType::Manifestation(el, _, _) => {
                format!("{} Manifestation", el.to_str())
            }
            DevelopmentSkillType::Resistance(el, _) => format!("{} Resistance", el.to_str()),
            DevelopmentSkillType::WillOf(el, _) => format!("Will of {}", el.to_str()),
            DevelopmentSkillType::Bravery(_) => String::from("Bravery"),
            DevelopmentSkillType::Encouragement => String::from("Encouragement"),
            DevelopmentSkillType::Blessing => String::from("Blessing"),
            DevelopmentSkillType::Flashback => String::from("Flashback"),
            DevelopmentSkillType::Hex(_) => String::from("Hex"),
            DevelopmentSkillType::MartialArts(_) => String::from("Martial Arts"),
            DevelopmentSkillType::Tattletale(_) => String::from("Tattletale"),
            DevelopmentSkillType::FightingSpirit(_) => String::from("Fighting Spirit"),
            DevelopmentSkillType::BravePerformance(_) => String::from("Brave Performance"),
            DevelopmentSkillType::BattleBravery(_) => String::from("Battle Bravery"),
            DevelopmentSkillType::Rigid(_) => String::from("Rigid"),
            DevelopmentSkillType::Forestall(_) => String::from("Forestall"),
            DevelopmentSkillType::BattleArts(_) => String::from("Battle Arts"),
            DevelopmentSkillType::Concentrate(_) => String::from("Concentrate"),
            DevelopmentSkillType::Instinct(_) => String::from("Instinct"),
            DevelopmentSkillType::Climb(_) => String::from("Climb"),
            DevelopmentSkillType::Crush(_) => String::from("Crush"),
            DevelopmentSkillType::FistStrike(_) => String::from("Fist Strike"),
            DevelopmentSkillType::Mage(_) => String::from("Mage"),
            DevelopmentSkillType::MindsEye(_) => String::from("Mind's Eye"),
            DevelopmentSkillType::Acceleration(_) => String::from("Acceleration"),
            DevelopmentSkillType::Hunter(_) => String::from("Hunter"),
            DevelopmentSkillType::Crafter(_) => String::from("Crafter"),
            DevelopmentSkillType::Protection(_) => String::from("Protection"),
            DevelopmentSkillType::MagicResistance(_) => String::from("Magic Resistance"),
            DevelopmentSkillType::StatusResist(_) => String::from("Status Resist"),
            DevelopmentSkillType::AbnormalResistance(_) => String::from("Abnormal Resistance"),
            DevelopmentSkillType::Solid(_) => String::from("Solid"),
            DevelopmentSkillType::Strike(_) => String::from("Strike"),
            DevelopmentSkillType::PiercingStrike(_) => String::from("Piercing Strike"),
            DevelopmentSkillType::TrueStrike(_) => String::from("True Strike"),
            DevelopmentSkillType::CounterAttack(_) => String::from("Counter-attack"),
            DevelopmentSkillType::Bloom(_) => String::from("Bloom"),
            DevelopmentSkillType::SpiritHealing(_) => String::from("Spirit Healing"),
            DevelopmentSkillType::LiarisFreese => String::from("Liaris Freese"),
            DevelopmentSkillType::Luck(_) => String::from("Luck"),
            DevelopmentSkillType::Killer(x) => {
                if x == &EnemyType::Ox {
                    format!("{} Slayer", x.to_str())
                } else {
                    format!("{} Killer", x.to_str())
                }
            }
            DevelopmentSkillType::Unknown(title) => title.to_string(),
        }
    }
}

fn from_strs(strs: Vec<&str>) -> Vec<String> {
    let mut res = Vec::new();
    for str in strs {
        res.push(String::from(str));
    }
    res
}

fn multiply_vec(v: &mut Vec<String>, n: i32) {
    if n < 2 {
        return;
    }

    let first = v.first().unwrap().to_string();
    for _ in 0..(n - 1) {
        v.push(first.clone());
    }
}

impl DevelopmentSkillType {
    pub fn str_to_type(orig_inp: &str) -> DevelopmentSkillType {
        let mut inp = orig_inp.trim().to_string();
        if inp.contains(":") {
            inp = inp.split(":").nth(0).unwrap().trim().to_string();
        }
        let inp_lower = inp.to_lowercase();

        // Check if it is a killer / slayer skill
        if inp_lower.contains("killer") || inp_lower.contains("slayer") {
            let o_et = inp_lower.split(" ").nth(0);
            if let Some(mut et_str) = o_et {
                et_str = et_str.trim();
                if let Some(et) = EnemyType::str_to_type(et_str) {
                    let st = inp_lower.split(" ").nth(1).unwrap().trim();
                    if (st == "killer" && et != EnemyType::Ox)
                        || (st == "slayer" && et == EnemyType::Ox)
                    {
                        return DevelopmentSkillType::Killer(et);
                    }
                }
            }
        }
        // check if it is a skill whose name starts with an element
        if inp_lower.contains("manifestation") || inp_lower.contains("resistance") {
            let o_et = inp_lower.split(" ").nth(0);
            if let Some(mut el_str) = o_et {
                el_str = el_str.trim();
                if let Some(el) = Element::str_to_type(el_str) {
                    let o_st = inp_lower.split(" ").nth(1);
                    if let Some(mut st) = o_st {
                        st = st.trim();
                        if st == "manifestation" {
                            return DevelopmentSkillType::Manifestation(
                                el,
                                DamageType::Physical,
                                0,
                            );
                        } else if st == "resistance" {
                            return DevelopmentSkillType::Resistance(el, 0);
                        }
                    }
                }
            }
        }
        // Will Of Element??
        if inp_lower.contains("will of") {
            let o_et = inp_lower.split(" ").nth(2);
            if let Some(mut et_str) = o_et {
                et_str = et_str.trim();
                if let Some(et) = Element::str_to_type(et_str) {
                    let first = inp_lower.split(" ").nth(0).unwrap().trim();
                    let second = inp_lower.split(" ").nth(1).unwrap().trim();
                    if first == "will" && second == "of" {
                        return DevelopmentSkillType::WillOf(et, DamageType::Physical);
                    }
                }
            }
        }

        let ty = match inp_lower.as_str() {
            "bravery" => DevelopmentSkillType::Bravery(0),
            "encouragement" => DevelopmentSkillType::Encouragement,
            "blessing" => DevelopmentSkillType::Blessing,
            "flashback" => DevelopmentSkillType::Flashback,
            "hex" => DevelopmentSkillType::Hex(0),
            "martial arts" => DevelopmentSkillType::MartialArts(0),
            "tattletale" => DevelopmentSkillType::Tattletale(0),
            "fighting spirit" => DevelopmentSkillType::FightingSpirit(0),
            "battle bravery" => DevelopmentSkillType::BattleBravery(0),
            "rigid" => DevelopmentSkillType::Rigid(0),
            "forestall" => DevelopmentSkillType::Forestall(0),
            "battle arts" => DevelopmentSkillType::BattleArts(0),
            "concentrate" => DevelopmentSkillType::Concentrate(0),
            "instinct" => DevelopmentSkillType::Instinct(0),
            "climb" => DevelopmentSkillType::Climb(0),
            "crush" => DevelopmentSkillType::Crush(0),
            "fist strike" => DevelopmentSkillType::FistStrike(0),
            "mage" => DevelopmentSkillType::Mage(0),
            "mind's eye" => DevelopmentSkillType::MindsEye(0),
            "minds eye" => DevelopmentSkillType::MindsEye(0),
            "acceleration" => DevelopmentSkillType::Acceleration(0),
            "hunter" => DevelopmentSkillType::Hunter(0),
            "crafter" => DevelopmentSkillType::Crafter(0),
            "protection" => DevelopmentSkillType::Protection(0),
            "magic resistance" => DevelopmentSkillType::MagicResistance(0),
            "status resist" => DevelopmentSkillType::StatusResist(0),
            "abnormal resistance" => DevelopmentSkillType::AbnormalResistance(0),
            "solid" => DevelopmentSkillType::Solid(0),
            "strike" => DevelopmentSkillType::Strike(0),
            "piercing strike" => DevelopmentSkillType::PiercingStrike(0),
            "true strike" => DevelopmentSkillType::TrueStrike(0),
            "counter-attack" => DevelopmentSkillType::CounterAttack(0),
            "counter attack" => DevelopmentSkillType::CounterAttack(0),
            "counterattack" => DevelopmentSkillType::CounterAttack(0),
            "bloom" => DevelopmentSkillType::Bloom(0),
            "spirit healing" => DevelopmentSkillType::SpiritHealing(0),
            "liaris freese" => DevelopmentSkillType::LiarisFreese,
            "luck" => DevelopmentSkillType::Luck(0),
            _ => DevelopmentSkillType::Unknown(inp.to_string()),
        };

        ty
    }

    // Modifier, Target, Attribute string
    // pub fn get_triples(&self) -> Vec<(String, &str, String)> {
    //     use Attribute::*;
    //     use DevelopmentSkillType::*;
    //     let triples = vec![(String::from(""), "", String::from(""))];
    //     let triples = match self {
    //         Manifestation(el, dt, modi) => vec![
    //             (String::from(""), "None", format!("When countering and attacking, regular {}Attack a Foe with {} Element", dt.to_short_str(), el.to_str())),
    //             (format!("+{}", modi), Target::Auto.to_json(), format!("{}_resist", el.effective_against().to_json()))],
    //         Resistance(el, modi) => vec![(format!("+{}", modi), Target::Auto.to_json(), format!("{}_resist", el.to_json()))],
    //         WillOf(el, dt) => vec![(String::from(""), "None", format!("When countering and attacking, regular {}Attack a Foe with {} Element", dt.to_short_str(), el.to_str()))],
    //         Bravery(modi) => vec![
    //             (String::from(""), "None", String::from("When countering and attacking, regular attack a Foe")),
    //             (format!("+{}", modi), "counter", String::from("life_steal"))],
    //         Encouragement => vec![(String::from(""), "None", String::from("When countering, instead of attacking regularly, it (Lo) Heals an Ally. Prioritizes an Ally with lower percentage HP."))],
    //         Blessing => vec![String::from("When countering, instead of attacking regularly, it extends Status Buff for Allies for 1 turn.")],
    //         Flashback => vec![String::from("Ultra Critical Rate on Counter")],
    //         Hex(_) => from_strs(vec![
    //             Magic.to_json(),
    //             Endurance.to_json(),
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         MartialArts(_) => from_strs(vec![
    //             Strength.to_json(),
    //             Endurance.to_json(),
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         Tattletale(_) => from_strs(vec![
    //             Magic.to_json(),
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         FightingSpirit(_) => from_strs(vec![
    //             Strength.to_json(),
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         Rigid(_) => from_strs(vec![
    //             Endurance.to_json(),
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         Forestall(_) => from_strs(vec![
    //             Magic.to_json(),
    //             Agility.to_json()]),
    //         BattleArts(_) => from_strs(vec![
    //             Strength.to_json(),
    //             Dexterity.to_json()]),
    //         Concentrate(_) => from_strs(vec![
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         Instinct(_) => from_strs(vec![
    //             Agility.to_json(),
    //             Dexterity.to_json()]),
    //         Climb(_) => from_strs(vec![
    //             Endurance.to_json(),
    //             Agility.to_json()]),
    //         Crush(_) => from_strs(vec![Strength.to_json()]),
    //         FistStrike(_) => from_strs(vec![Strength.to_json()]),
    //         Mage(_) => from_strs(vec![Magic.to_json()]),
    //         MindsEye(_) => from_strs(vec![Magic.to_json()]),
    //         Acceleration(_) => from_strs(vec![Agility.to_json()]),
    //         Hunter(_) => from_strs(vec![Agility.to_json()]),
    //         Crafter(_) => from_strs(vec![Dexterity.to_json()]),
    //         Protection(_) => from_strs(vec![
    //             PhysicalResistance.to_json(),
    //             Attribute::MagicResistance.to_json()]),
    //         DevelopmentSkillType::MagicResistance(_) => from_strs(vec![Attribute::MagicResistance.to_json()]),
    //         StatusResist(_) => from_strs(vec![AilmentResist.to_json()]),
    //         AbnormalResistance(_) => from_strs(vec![AilmentResist.to_json()]),
    //         Solid(_) => from_strs(vec![GuardRate.to_json()]),
    //         Strike(_) => from_strs(vec![CriticalDamage.to_json()]),
    //         PiercingStrike(_) => from_strs(vec![PenetrationDamage.to_json()]),
    //         TrueStrike(_) => from_strs(vec![
    //             CriticalDamage.to_json(),
    //             PenetrationDamage.to_json()]),
    //         CounterAttack(_) => from_strs(vec![CounterDamage.to_json()]),
    //         Bloom(_) => from_strs(vec![
    //             HPRegen.to_json(),
    //             MPRegen.to_json()]),
    //         SpiritHealing(_) => from_strs(vec![MPRegen.to_json()]),
    //         LiarisFreese => from_strs(vec![
    //             "fast_growth",
    //             "null_charm"]),
    //         Luck(_) => from_strs(vec!["All Status"]),
    //         Killer(et) => {
    //             if et == &EnemyType::Ox {
    //                 vec![format!("Ability Pt. toward {}", et.to_desc_str())]
    //             } else {
    //                 vec![format!("Ability Pt. toward {}", et.to_desc_str())]
    //             }
    //         }
    //         Unknown(_) => vec![String::from("")],
    //     };

    //     triples
    // }

    pub fn get_descriptions(&self) -> Vec<String> {
        use Attribute::*;
        use DevelopmentSkillType::*;
        let descs = match self {
            Manifestation(el, dt, _) => vec![
                format!("When countering and attacking, regular {}Attack a Foe with {} Element", dt.to_short_str(), el.to_str()),
                format!("{}_resist", el.effective_against().to_json())],
            Resistance(el, _) => vec![format!("{}_resist", el.to_json())],
            WillOf(el, dt) => vec![format!("When countering and attacking, regular {}Attack a Foe with {} Element", dt.to_short_str(), el.to_str())],
            Bravery(_) => from_strs(vec![
                "When countering and attacking, regular attack a Foe",
                "life_steal"]),
            Encouragement => vec![String::from("When countering, instead of attacking regularly, it (Lo) Heals an Ally. Prioritizes an Ally with lower percentage HP.")],
            Blessing => vec![String::from("When countering, instead of attacking regularly, it extends Status Buff for Allies for 1 turn.")],
            Flashback => vec![String::from("Ultra Critical Rate on Counter")],
            Hex(_) => from_strs(vec![
                Magic.to_json(),
                Endurance.to_json(),
                Agility.to_json(),
                Dexterity.to_json()]),
            MartialArts(_) => from_strs(vec![
                Strength.to_json(),
                Endurance.to_json(),
                Agility.to_json(),
                Dexterity.to_json()]),
            Tattletale(_) => from_strs(vec![
                Magic.to_json(),
                Agility.to_json(),
                Dexterity.to_json()]),
            FightingSpirit(_) => from_strs(vec![
                Strength.to_json(),
                Agility.to_json(),
                Dexterity.to_json()]),
            BravePerformance(_) => from_strs(vec![
                Magic.to_json(),
                Endurance.to_json(),
                Dexterity.to_json()]),
            BattleBravery(_) => from_strs(vec![
                Strength.to_json(),
                Endurance.to_json(),
                Dexterity.to_json()]),
            Rigid(_) => from_strs(vec![
                Endurance.to_json(),
                Agility.to_json(),
                Dexterity.to_json()]),
            Forestall(_) => from_strs(vec![
                Magic.to_json(),
                Agility.to_json()]),
            BattleArts(_) => from_strs(vec![
                Strength.to_json(),
                Dexterity.to_json()]),
            Concentrate(_) => from_strs(vec![
                Agility.to_json(),
                Dexterity.to_json()]),
            Instinct(_) => from_strs(vec![
                Agility.to_json(),
                Dexterity.to_json()]),
            Climb(_) => from_strs(vec![
                Endurance.to_json(),
                Agility.to_json()]),
            Crush(_) => from_strs(vec![Strength.to_json()]),
            FistStrike(_) => from_strs(vec![Strength.to_json()]),
            Mage(_) => from_strs(vec![Magic.to_json()]),
            MindsEye(_) => from_strs(vec![Magic.to_json()]),
            Acceleration(_) => from_strs(vec![Agility.to_json()]),
            Hunter(_) => from_strs(vec![Agility.to_json()]),
            Crafter(_) => from_strs(vec![Dexterity.to_json()]),
            Protection(_) => from_strs(vec![
                PhysicalResistance.to_json(),
                Attribute::MagicResistance.to_json()]),
            DevelopmentSkillType::MagicResistance(_) => from_strs(vec![Attribute::MagicResistance.to_json()]),
            StatusResist(_) => from_strs(vec![AilmentResist.to_json()]),
            AbnormalResistance(_) => from_strs(vec![AilmentResist.to_json()]),
            Solid(_) => from_strs(vec![GuardRate.to_json()]),
            Strike(_) => from_strs(vec![CriticalDamage.to_json()]),
            PiercingStrike(_) => from_strs(vec![PenetrationDamage.to_json()]),
            TrueStrike(_) => from_strs(vec![
                CriticalDamage.to_json(),
                PenetrationDamage.to_json()]),
            CounterAttack(_) => from_strs(vec![CounterDamage.to_json()]),
            Bloom(_) => from_strs(vec![
                HPRegen.to_json(),
                MPRegen.to_json()]),
            SpiritHealing(_) => from_strs(vec![MPRegen.to_json()]),
            LiarisFreese => from_strs(vec![
                "fast_growth",
                "null_charm"]),
            Luck(_) => from_strs(vec!["All Status"]),
            Killer(et) => {
                if et == &EnemyType::Ox {
                    vec![format!("Ability Pt. toward {}", et.to_desc_str())]
                } else {
                    vec![format!("Ability Pt. toward {}", et.to_desc_str())]
                }
            }
            Unknown(_) => vec![String::from("")],
        };

        descs
    }

    pub fn get_modifiers(&self) -> Vec<String> {
        use DevelopmentSkillType::*;

        let mut modis = match self {
            Manifestation(_, _, modi) => {
                vec![String::from(""), format!("+{}", modi)]
            }
            Bravery(modi) => vec![String::from(""), modi.to_string()],
            WillOf(_, _) | Encouragement | Blessing | Flashback | LiarisFreese | Unknown(_) => {
                vec![String::from("")]
            }
            Resistance(_, modi)
            | Hex(modi)
            | MartialArts(modi)
            | Tattletale(modi)
            | FightingSpirit(modi)
            | BravePerformance(modi)
            | BattleBravery(modi)
            | Rigid(modi)
            | Forestall(modi)
            | BattleArts(modi)
            | Concentrate(modi)
            | Instinct(modi)
            | Climb(modi)
            | Crush(modi)
            | FistStrike(modi)
            | Mage(modi)
            | MindsEye(modi)
            | Acceleration(modi)
            | Hunter(modi)
            | Crafter(modi)
            | Protection(modi)
            | MagicResistance(modi)
            | StatusResist(modi)
            | AbnormalResistance(modi)
            | Solid(modi)
            | Strike(modi)
            | PiercingStrike(modi)
            | TrueStrike(modi)
            | CounterAttack(modi)
            | Luck(modi) => vec![format!("+{}", modi)],
            Bloom(modi) | SpiritHealing(modi) => vec![modi.to_string()],
            Killer(et) => {
                if et == &EnemyType::Ox {
                    vec![String::from("+100")]
                } else {
                    vec![String::from("+50")]
                }
            }
        };

        let n = match self {
            Hex(_) | MartialArts(_) => 4,
            Tattletale(_) | FightingSpirit(_) | BravePerformance(_) | BattleBravery(_)
            | Rigid(_) => 3,
            Forestall(_) | BattleArts(_) | Concentrate(_) | Instinct(_) | Climb(_)
            | Protection(_) | TrueStrike(_) | Bloom(_) | LiarisFreese => 2,
            _ => 1,
        };
        multiply_vec(&mut modis, n);

        modis
    }
}

#[derive(PartialEq, Eq)]
pub enum SkillRank {
    Question,
    I,
    H,
    G,
    F,
    E,
    D,
    C,
    B,
    A,
}

impl HumanReadable for SkillRank {
    fn to_str(&self) -> String {
        match self {
            SkillRank::Question => "?",
            SkillRank::I => "I",
            SkillRank::H => "H",
            SkillRank::G => "G",
            SkillRank::F => "F",
            SkillRank::E => "E",
            SkillRank::D => "D",
            SkillRank::C => "C",
            SkillRank::B => "B",
            SkillRank::A => "A",
        }
        .to_string()
    }
}

impl SkillRank {
    pub fn str_to_type(inp: &str) -> Option<SkillRank> {
        let inp = &inp.trim().to_lowercase();
        Some(match inp.as_str() {
            "?" => SkillRank::Question,
            "i" => SkillRank::I,
            "h" => SkillRank::H,
            "g" => SkillRank::G,
            "f" => SkillRank::F,
            "e" => SkillRank::E,
            "d" => SkillRank::D,
            "c" => SkillRank::C,
            "b" => SkillRank::B,
            "a" => SkillRank::A,
            _ => return None,
        })
    }
}

#[derive(PartialEq, Eq)]
pub enum EnemyType {
    Dragon,
    Spirit,
    Giant,
    Ox,
    Beast,
    Ogre,
    Insect,
    Plant,
    Aqua,
    Material,
    Worm,
    Fantasma,
}

impl HumanReadable for EnemyType {
    fn to_str(&self) -> String {
        match self {
            EnemyType::Dragon => "Dragon",
            EnemyType::Spirit => "Spirit",
            EnemyType::Giant => "Giant",
            EnemyType::Ox => "Ox",
            EnemyType::Beast => "Beast",
            EnemyType::Ogre => "Ogre",
            EnemyType::Insect => "Insect",
            EnemyType::Plant => "Plant",
            EnemyType::Aqua => "Aqua",
            EnemyType::Material => "Material",
            EnemyType::Worm => "Worm",
            EnemyType::Fantasma => "Fantasma",
        }
        .to_string()
    }
}

impl EnemyType {
    pub fn str_to_type(inp: &str) -> Option<EnemyType> {
        let inp = &inp.trim().to_lowercase();
        match inp.as_str() {
            "dragon" => Some(EnemyType::Dragon),
            "spirit" => Some(EnemyType::Spirit),
            "giant" => Some(EnemyType::Giant),
            "ox" => Some(EnemyType::Ox),
            "beast" => Some(EnemyType::Beast),
            "ogre" => Some(EnemyType::Ogre),
            "insect" => Some(EnemyType::Insect),
            "plant" => Some(EnemyType::Plant),
            "aqua" => Some(EnemyType::Aqua),
            "material" => Some(EnemyType::Material),
            "worm" => Some(EnemyType::Worm),
            "fantasma" => Some(EnemyType::Fantasma),
            _ => None,
        }
    }

    pub fn to_desc_str(&self) -> String {
        match self {
            EnemyType::Dragon => "Dragons",
            EnemyType::Spirit => "Spirits",
            EnemyType::Giant => "Giants",
            EnemyType::Ox => "Ox",
            EnemyType::Beast => "Beasts",
            EnemyType::Ogre => "Ogres",
            EnemyType::Insect => "Insects",
            EnemyType::Plant => "Plants",
            EnemyType::Aqua => "Aqua",
            EnemyType::Material => "Material",
            EnemyType::Worm => "Worms",
            EnemyType::Fantasma => "Ghost",
        }
        .to_string()
    }
}
