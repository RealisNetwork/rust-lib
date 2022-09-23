// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for LobbyStatsOptionsEndpointsGetByPersonalTypeParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\"]}")
    }
}
impl Agent for LobbyStatsOptionsEndpointsGetByPersonalTypeParams {
    fn topic() -> &'static str {
        "lobby_statsOptionsEndpoints_getByPersonalType"
    }
    fn method() -> &'static str {
        "statsOptionsEndpoints_getByPersonalType"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsAttributesRangesParams {
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<f64>,
    #[serde(rename = "strength")]
    pub strength: Vec<f64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<f64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<f64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<f64>,
    #[serde(rename = "health")]
    pub health: Vec<f64>,
    #[serde(rename = "armor")]
    pub armor: Vec<f64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<f64>,
    #[serde(rename = "agility")]
    pub agility: Vec<f64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<f64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<f64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<f64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<f64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsMultipliersRangesParams {
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<f64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<f64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<f64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<f64>,
    #[serde(rename = "agility")]
    pub agility: Vec<f64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<f64>,
    #[serde(rename = "armor")]
    pub armor: Vec<f64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<f64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<f64>,
    #[serde(rename = "health")]
    pub health: Vec<f64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<f64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<f64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<f64>,
    #[serde(rename = "strength")]
    pub strength: Vec<f64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams
{
    #[serde(rename = "agility")]
    pub agility: f64,
    #[serde(rename = "intelligence")]
    pub intelligence: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
    #[serde(rename = "skillPower")]
    pub skill_power: f64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: f64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: f64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: f64,
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "ultPower")]
    pub ult_power: f64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
    #[serde(rename = "armor")]
    pub armor: f64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams
{
    #[serde(rename = "coefficient")]
    pub coefficient: f64,
    #[serde(rename = "level")]
    pub level: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams
{
    #[serde(rename = "level")]
    pub level: f64,
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParams { # [serde (rename = "maxLevel")] pub max_level : f64 , # [serde (rename = "attributesPerLevel")] pub attributes_per_level : LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams , # [serde (rename = "baseScrollsCount")] pub base_scrolls_count : f64 , # [serde (rename = "experienceCoefficients")] pub experience_coefficients : Vec < LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams > , # [serde (rename = "baseExperience")] pub base_experience : f64 , # [serde (rename = "intelligence")] pub intelligence : f64 , # [serde (rename = "agility")] pub agility : f64 , # [serde (rename = "experience")] pub experience : f64 , # [serde (rename = "strength")] pub strength : f64 , # [serde (rename = "scrollsByLevels")] pub scrolls_by_levels : Vec < LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams > }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturns {
    #[serde(rename = "attributesRanges")]
    pub attributes_ranges: LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsAttributesRangesParams,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: String,
    #[serde(rename = "multipliersRanges")]
    pub multipliers_ranges:
        Option<LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsMultipliersRangesParams>,
    #[serde(rename = "levelUpOptions")]
    pub level_up_options: LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParams,
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for LobbyStatsOptionsEndpointsGetByPersonalTypeReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"attributesRanges\":{\"type\":\"object\",\"properties\":{\"skillPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"strength\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"moveSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"healthRegenPercent\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"intelligence\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"health\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"armor\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackDamage\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"agility\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackReloadSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"vampirismPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"mainCharacteristic\":{\"type\":\"string\"},\"multipliersRanges\":{\"type\":\"object\",\"properties\":{\"moveSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackDamage\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"skillPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"agility\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"vampirismPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"armor\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"healthRegenPercent\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"health\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"ultEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"intelligence\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"attackReloadSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"strength\":{\"type\":\"array\",\"items\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"levelUpOptions\":{\"type\":\"object\",\"properties\":{\"maxLevel\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attributesPerLevel\":{\"type\":\"object\",\"properties\":{\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"baseScrollsCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"experienceCoefficients\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"coefficient\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"level\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"coefficient\",\"level\"]}},\"baseExperience\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"scrollsByLevels\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"level\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"scrollsCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"scrollsCount\",\"level\"]}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"attributesPerLevel\",\"experience\",\"experienceCoefficients\",\"baseExperience\",\"baseScrollsCount\",\"scrollsByLevels\",\"maxLevel\"]},\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\",\"mainCharacteristic\",\"levelUpOptions\",\"attributesRanges\"]}")
    }
}
impl Agent for LobbyStatsOptionsEndpointsGetByPersonalTypeReturns {
    fn topic() -> &'static str {
        "lobby_statsOptionsEndpoints_getByPersonalType"
    }
    fn method() -> &'static str {
        "statsOptionsEndpoints_getByPersonalType"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
