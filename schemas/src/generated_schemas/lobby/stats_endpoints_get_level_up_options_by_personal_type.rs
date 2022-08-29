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
    #[serde(rename = "coefficient")]
    pub coefficient: String,
    #[serde(rename = "level")]
    pub level: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsScrollsByLevelsParamsParams {
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsAttributesPerLevelParams {
    #[serde(rename = "skillPower")]
    pub skill_power: f64,
    #[serde(rename = "intelligence")]
    pub intelligence: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
    #[serde(rename = "agility")]
    pub agility: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: f64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: f64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: f64,
    #[serde(rename = "armor")]
    pub armor: f64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: f64,
    #[serde(rename = "ultPower")]
    pub ult_power: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturns {
    #[serde(rename = "baseScrollsCount")]
    pub base_scrolls_count: i32,
    #[serde(rename = "experienceCoefficients")]
    pub experience_coefficients: Vec<
        LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsExperienceCoefficientsParamsParams,
    >,
    #[serde(rename = "maxLevel")]
    pub max_level: i32,
    #[serde(rename = "baseExperience")]
    pub base_experience: i32,
    #[serde(rename = "scrollsByLevels")]
    pub scrolls_by_levels:
        Vec<LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsScrollsByLevelsParamsParams>,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "attributesPerLevel")]
    pub attributes_per_level:
        LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturnsAttributesPerLevelParams,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "experience")]
    pub experience: i32,
}
impl Schema for LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"baseScrollsCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experienceCoefficients\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"coefficient\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"coefficient\",\"level\"]}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"baseExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"scrollsByLevels\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"scrollsCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"scrollsCount\",\"level\"]}},\"intelligence\":{\"type\":\"string\"},\"attributesPerLevel\":{\"type\":\"object\",\"properties\":{\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"agility\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"attributesPerLevel\",\"experience\",\"experienceCoefficients\",\"baseExperience\",\"baseScrollsCount\",\"scrollsByLevels\",\"maxLevel\"]}")
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
