pub const HEADER1: &str = "{
    \"title\": \"$0\",
    \"name\": \"$1\",";

pub const ATTACKTYPE: &str = "
    \"type\": \"$0\",";

pub const HEADER2: &str = "
    \"stars\": $0,
    \"limited\": $1,";

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
/*                {
\"target\": \"foes/allies/self\",
\"attribute\": \"second_attribute\",
\"modifier\": \"-+0\"*/
pub const ASS_FOOT_SKILL_ONE: &str = "
            ]
        },";
pub const ASS_SKILL_TWO_HEAD: &str = "
        {
            \"name\": \"$0++\",
            \"effects\": [";
/*                {
    \"target\": \"foes/allies/self\",
    \"attribute\": \"first_attribute\",
    \"modifier\": \"-+0\"
},
{
    \"target\": \"foes/allies/self\",
    \"attribute\": \"second_attribute\",
    \"modifier\": \"-+0\"*/
pub const ASS_FOOT: &str = "
            ]
        }
    ]
}";

pub const ADVSA: &str = "
    \"skills\": {
        \"special\": {
            \"name\": \"sa-skill-name:\",
            \"effects\": [
                {
                    \"target\": \"foes/foe (if applicable)\",
                    \"modifier\": \"ultra/super etc.\",
                    \"element\": \"$0\",
                    \"type\": \"$1\"
                },
                {
                    \"target\":\"skill\",
                    \"modifier\":\"great_mag_temp/great_str_temp/mag_temp/str_temp (if applicable)\"
                },
                {
                    \"target\": \"skill\",
                    \"attribute\": \"penetration_rate/unguard_rate etc. (if applicable)\",
                    \"modifier\": \"high/ultra etc.\"
                },
                {
                    \"target\": \"self/allies/foe/foes\",
                    \"duration\": \"0\",
                    \"attribute\": \"attribute-name\",
                    \"modifier\": \"-+0\"
                }
            ]
        },
        \"combat\": [";

pub const ADVSKILL: &str = "
            {
                \"name\": \"skill-name:\",
                \"effects\": [
                    {
                        \"target\": \"self/allies/foe/foes\",
                        \"speed\": \"fast/slow/none\",
                        \"modifier\": \"low/medium/high/super\",
                        \"element\": \"$0\",
                        \"type\": \"$1\"
                    },
                    {
                        \"target\":\"skill\",
                        \"modifier\":\"great_mag_temp/great_str_temp/mag_temp/str_temp (if applicable)\"
                    },
                    {
                        \"target\": \"skill\",
                        \"attribute\": \"penetration_rate/unguard_rate etc. (if applicable)\",
                        \"modifier\": \"high/ultra etc.\"
                    },
                    {
                        \"target\": \"self/allies/foe/foes\",
                        \"attribute\": \"\",
                        \"modifier\": \"-+0\",
                        \"duration\": \"0\"
                    },
                    {
                        \"target\":\"self\",
                        \"duration\":\"+number-of-actions\",
                        \"attribute\":\"Additional action (if applicable, just paste full text in these braces)\"
                    }
                ]";

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
