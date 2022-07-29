// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for LobbyStatsEndpointsGetStatsOptionsByPersonalTypeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}")
    }
}
impl Agent for LobbyStatsEndpointsGetStatsOptionsByPersonalTypeParams {
    fn topic() -> &'static str {
        "lobby_statsEndpoints_getStatsOptionsByPersonalType"
    }
    fn method() -> &'static str {
        "statsEndpoints_getStatsOptionsByPersonalType"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams
{
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams
{
    #[serde(rename = "coefficient")]
    pub coefficient: String,
    #[serde(rename = "level")]
    pub level: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams
{
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: i32,
    #[serde(rename = "level")]
    pub level: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParams { # [serde (rename = "baseScrollsCount")] pub base_scrolls_count : i32 , # [serde (rename = "attributesPerLevel")] pub attributes_per_level : LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams , # [serde (rename = "maxLevel")] pub max_level : i32 , # [serde (rename = "experienceCoefficients")] pub experience_coefficients : Vec < LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams > , # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "agility")] pub agility : String , # [serde (rename = "baseExperience")] pub base_experience : i32 , # [serde (rename = "scrollsByLevels")] pub scrolls_by_levels : Vec < LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams > }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsStatsRangesParams {
    #[serde(rename = "strength")]
    pub strength: Vec<i64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<i64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<i64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<i64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<i64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<i64>,
    #[serde(rename = "health")]
    pub health: Vec<i64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<i64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<i64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<i64>,
    #[serde(rename = "agility")]
    pub agility: Vec<i64>,
    #[serde(rename = "armor")]
    pub armor: Vec<i64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<i64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturns {
    #[serde(rename = "levelUpOptions")]
    pub level_up_options:
        LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParams,
    #[serde(rename = "statsRanges")]
    pub stats_ranges: LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsStatsRangesParams,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: String,
}
impl Schema for LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"levelUpOptions\":{\"type\":\"object\",\"properties\":{\"baseScrollsCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attributesPerLevel\":{\"type\":\"object\",\"properties\":{\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experienceCoefficients\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"coefficient\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"coefficient\",\"level\"]}},\"intelligence\":{\"type\":\"string\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"strength\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"baseExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"scrollsByLevels\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"scrollsCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"scrollsCount\",\"level\"]}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"attributesPerLevel\",\"experience\",\"experienceCoefficients\",\"baseExperience\",\"baseScrollsCount\",\"scrollsByLevels\",\"maxLevel\"]},\"statsRanges\":{\"type\":\"object\",\"properties\":{\"strength\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"vampirismPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackDamage\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"intelligence\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"health\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackReloadSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"agility\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"armor\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"moveSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"healthRegenPercent\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"mainCharacteristic\":{\"type\":\"string\"}},\"required\":[\"levelUpOptions\",\"statsRanges\",\"mainCharacteristic\"]}")
    }
}
impl Agent for LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturns {
    fn topic() -> &'static str {
        "lobby_statsEndpoints_getStatsOptionsByPersonalType"
    }
    fn method() -> &'static str {
        "statsEndpoints_getStatsOptionsByPersonalType"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
