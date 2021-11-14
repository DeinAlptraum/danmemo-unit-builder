// Header
pub const HEADER1: &str = "{
    \"title\": \"$0\",
    \"name\": \"$1\",";

pub const ATTACKTYPE: &str = "
    \"type\": \"$0\",";

pub const HEADER2: &str = "
    \"stars\": $0,
    \"limited\": $1,";

// Stats
pub const STATS_HP: &str = "
    \"stats\": {
        \"hp\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";
pub const STATS_MP: &str = "
        \"mp\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_PAT: &str = "
        \"physical_attack\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_MAT: &str = "
        \"magic_attack\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_DEF: &str = "
        \"defense\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_STR: &str = "
        \"strength\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_END: &str = "
        \"endurance\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_DEX: &str = "
        \"dexterity\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_AGI: &str = "
        \"agility\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ],";

pub const STATS_MAG: &str = "
        \"magic\": [
            $0,
            $1,
            $2,
            $3,
            $4,
            $5
        ]
    },";

// Assist Skills
pub const ASS_SKILLS_HEAD: &str = "    
    \"skills\": [
        {
            \"name\": \"$0+\",
            \"effects\": [";
pub const ASS_EFFECT: &str = "
                {
                    \"target\": \"$0\",
                    \"attribute\": \"$1\",
                    \"modifier\": \"$2\"
                }";

pub const ASS_FOOT_SKILL_ONE: &str = "
            ]
        },";
pub const ASS_SKILL_TWO_HEAD: &str = "
        {
            \"name\": \"$0++\",
            \"effects\": [";

pub const ASS_FOOT: &str = "
            ]
        }
    ]
}";

// Adventurer Skills
// SA Skill
pub const SAHEADER: &str = "
    \"skills\": {
        \"special\": {
            \"name\": \"$0:\",
            \"effects\": [";

// Dmg Effect
pub const SADMG: &str = "
                {
                    \"element\": \"$0\",
                    \"modifier\": \"$1\",
                    \"type\": \"$2\",
                    \"target\": \"$3\",
                    \"attribute\": \"None\",
                    \"speed\": \"None\"
                }";

pub const SATMPBOOST: &str = "
                {
                    \"modifier\":\"$0\",
                    \"target\":\"skill\",
                    \"attribute\": \"None\",
                    \"speed\": \"None\"
                }";

pub const SARATEBUFF: &str = "
                {
                    \"modifier\": \"ultra\",
                    \"target\": \"skill\",
                    \"attribute\": \"$0\",
                    \"speed\": \"None\"
                }";

pub const SALIFESTEAL: &str = "
                {
                    \"modifier\": \"$0\",
                    \"target\": \"skill\",
                    \"attribute\": \"life_steal\",
                    \"speed\": \"None\"
                }";

pub const SAPEB: &str = "
                {
                    \"modifier\": \"+$0\",
                    \"target\": \"skill\",
                    \"attribute\": \"$1\",
                    \"speed\": \"None\"
                }";

// Non-dmg skill effects
pub const SABUFF: &str = "
                {
                    \"duration\": \"$0\",
                    \"modifier\": \"$1\",
                    \"target\": \"$2\",
                    \"attribute\": \"$3\",
                    \"speed\": \"None\"
                }";

pub const SABUFFREMOVE: &str = "
                {
                    \"target\": \"$0\",
                    \"attribute\": \"$1\",
                    \"speed\": \"None\"
                }";

pub const SABUFFTURNS: &str = "
                {
                    \"duration\": \"$0\",
                    \"target\": \"$1\",
                    \"attribute\": \"$2\",
                    \"speed\": \"None\"
                }";

pub const SANULL: &str = "
                {
                    \"modifier\": \"$0\",
                    \"target\": \"$1\",
                    \"attribute\": \"$2\",
                    \"speed\": \"None\"
                }";

pub const SAHEAL: &str = "
                {
                    \"modifier\": \"$0\",
                    \"target\": \"$1\",
                    \"attribute\": \"$2\",
                    \"speed\": \"None\"
                }";

pub const SAAIL: &str = "
                {
                    \"modifier\": \"$0\",
                    \"target\": \"$1\",
                    \"attribute\": \"$2\",
                    \"speed\": \"None\"
                }";

pub const SAAILCURE: &str = "
                {
                    \"target\": \"allies\",
                    \"attribute\": \"ailment_cure\",
                    \"speed\": \"None\"
                }";

pub const SAFOOTER: &str = "
            ]
        },";

// Regular Skills
pub const COMBATHEADER: &str = "
        \"combat\": [";

pub const REGHEADER: &str = "
            {
                \"name\": \"$0:\"
                \"effects\": [";

// Dmg Effect
pub const REGDMG: &str = "
                    {
                        \"element\": \"$0\",
                        \"modifier\": \"$1\",
                        \"type\": \"$2\",
                        \"target\": \"$3\",
                        \"attribute\": \"None\",
                        \"speed\": \"$4\"
                    }";

pub const REGTMPBOOST: &str = "
                    {
                        \"modifier\":\"$0\",
                        \"target\":\"skill\",
                        \"attribute\": \"None\",
                        \"speed\": \"None\"
                    }";

pub const REGRATEBUFF: &str = "
                    {
                        \"modifier\": \"ultra\",
                        \"target\": \"skill\",
                        \"attribute\": \"$0\",
                        \"speed\": \"None\"
                    }";

pub const REGLIFESTEAL: &str = "
                    {
                        \"modifier\": \"$0\",
                        \"target\": \"skill\",
                        \"attribute\": \"life_steal\",
                        \"speed\": \"None\"
                    }";

pub const REGPEB: &str = "
                    {
                        \"modifier\": \"+$0\",
                        \"target\": \"skill\",
                        \"attribute\": \"$1\",
                        \"speed\": \"None\"
                    }";

// Non-dmg skill effects
pub const REGBUFF: &str = "
                    {
                        \"duration\": \"$0\",
                        \"modifier\": \"$1\",
                        \"target\": \"$2\",
                        \"attribute\": \"$3\",
                        \"speed\": \"$4\"
                    }";

pub const REGBUFFREMOVE: &str = "
                    {
                        \"target\": \"$0\",
                        \"attribute\": \"$1\",
                        \"speed\": \"$2\"
                    }";

pub const REGBUFFTURNS: &str = "
                    {
                        \"duration\": \"$0\",
                        \"target\": \"$1\",
                        \"attribute\": \"$2\",
                        \"speed\": \"$3\"
                    }";

pub const REGNULL: &str = "
                    {
                        \"modifier\": \"$0\",
                        \"target\": \"$1\",
                        \"attribute\": \"$2\",
                        \"speed\": \"$3\"
                    }";

pub const REGHEAL: &str = "
                    {
                        \"modifier\": \"$0\",
                        \"target\": \"$1\",
                        \"attribute\": \"$2\",
                        \"speed\": \"$3\"
                    }";

pub const REGAIL: &str = "
                    {
                        \"modifier\": \"$0\",
                        \"target\": \"$1\",
                        \"attribute\": \"$2\",
                        \"speed\": \"$3\"
                    }";

pub const REGAILCURE: &str = "
                    {
                        \"target\": \"allies\",
                        \"attribute\": \"ailment_cure\",
                        \"speed\": \"$0\"
                    }";

/*pub const REGADDACT: &str = "
                    {
                        \"target\":\"self\",
                        \"duration\":\"+number-of-actions\",
                        \"attribute\":\"Additional action (if applicable, just paste full text in these braces)\"
                    }";*/

pub const REGFOOTER: &str = "
                ]
            }";

pub const COMBATFOOTER: &str = "
        ],";

pub const DEVSKILLS: &str = "
            }
        ],
        \"development\": [
            {
                \"name\": \"skill-name\",
                \"effects\":[
                    {
                        \"attribute\": \"attribute-name\",
                        \"modifier\": \"+-0\"
                    }
                ]
            },
            {
                \"name\": \"skill-name2 \\n\",
                \"effects\": [
                    {
                        \"attribute\": \"Descrption-only attribute, e.g. counter skills. Just paste the full text\"
                    }
                ]
            },
            {
            \"name\": \"enemy-type Slayer \\n\",
                \"effects\": [
                    {
                        \"modifier\": \"+100\",
                        \"attribute\": \"Ability Pt. toward enemy-type\"
                    }
                ]
            }
        ]
    }
}";
