// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}")
    }
}
impl Agent for LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeParams {
    fn topic() -> &'static str {
        "lobby_statsEndpoints_getLevelUpOptionsByPersonalType"
    }
    fn method() -> &'static str {
        "statsEndpoints_getLevelUpOptionsByPersonalType"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsExperienceCoefficientsParamsParams
{
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "coefficient")]
    pub coefficient: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsScrollsByLevelsParamsParams {
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: i32,
    #[serde(rename = "level")]
    pub level: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsAttributesPerLevelParams {
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturns {
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "experienceCoefficients")]
    pub experience_coefficients: Vec<
        LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsExperienceCoefficientsParamsParams,
    >,
    #[serde(rename = "maxLevel")]
    pub max_level: i32,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "scrollsByLevels")]
    pub scrolls_by_levels:
        Vec<LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsScrollsByLevelsParamsParams>,
    #[serde(rename = "attributesPerLevel")]
    pub attributes_per_level:
        LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsAttributesPerLevelParams,
    #[serde(rename = "baseScrollsCount")]
    pub base_scrolls_count: i32,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "baseExperience")]
    pub base_experience: i32,
}
impl Schema for LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"strength\":{\"type\":\"string\"},\"experienceCoefficients\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"coefficient\":{\"type\":\"string\"}},\"required\":[\"coefficient\",\"level\"]}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"intelligence\":{\"type\":\"string\"},\"scrollsByLevels\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"scrollsCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"scrollsCount\",\"level\"]}},\"attributesPerLevel\":{\"type\":\"object\",\"properties\":{\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"baseScrollsCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"agility\":{\"type\":\"string\"},\"baseExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"attributesPerLevel\",\"experience\",\"experienceCoefficients\",\"baseExperience\",\"baseScrollsCount\",\"scrollsByLevels\",\"maxLevel\"]}")
    }
}
impl Agent for LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturns {
    fn topic() -> &'static str {
        "lobby_statsEndpoints_getLevelUpOptionsByPersonalType"
    }
    fn method() -> &'static str {
        "statsEndpoints_getLevelUpOptionsByPersonalType"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
