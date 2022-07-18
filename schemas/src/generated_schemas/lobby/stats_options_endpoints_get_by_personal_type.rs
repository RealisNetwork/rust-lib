// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams
{
    #[serde(rename = "level")]
    pub level: i64,
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams
{
    #[serde(rename = "level")]
    pub level: i64,
    #[serde(rename = "coefficient")]
    pub coefficient: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams
{
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParams { # [serde (rename = "baseExperience")] pub base_experience : i64 , # [serde (rename = "scrollsByLevels")] pub scrolls_by_levels : Vec < LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams > , # [serde (rename = "experienceCoefficients")] pub experience_coefficients : Vec < LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams > , # [serde (rename = "intelligence")] pub intelligence : i64 , # [serde (rename = "baseScrollsCount")] pub base_scrolls_count : i64 , # [serde (rename = "experience")] pub experience : i64 , # [serde (rename = "maxLevel")] pub max_level : i64 , # [serde (rename = "agility")] pub agility : i64 , # [serde (rename = "strength")] pub strength : i64 , # [serde (rename = "attributesPerLevel")] pub attributes_per_level : LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams }
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsAttributesRangesParams {
    #[serde(rename = "armor")]
    pub armor: Vec<i64>,
    #[serde(rename = "strength")]
    pub strength: Vec<i64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<i64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<i64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<i64>,
    #[serde(rename = "health")]
    pub health: Vec<i64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<i64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<i64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<i64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<i64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<i64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<i64>,
    #[serde(rename = "agility")]
    pub agility: Vec<i64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsMultipliersRangesParams {
    #[serde(rename = "strength")]
    pub strength: Vec<i64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<i64>,
    #[serde(rename = "agility")]
    pub agility: Vec<i64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<i64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<i64>,
    #[serde(rename = "armor")]
    pub armor: Vec<i64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<i64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<i64>,
    #[serde(rename = "health")]
    pub health: Vec<i64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<i64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<i64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<i64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<i64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsGetByPersonalTypeReturns {
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "levelUpOptions")]
    pub level_up_options: LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsLevelUpOptionsParams,
    #[serde(rename = "attributesRanges")]
    pub attributes_ranges: LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsAttributesRangesParams,
    #[serde(rename = "multipliersRanges")]
    pub multipliers_ranges:
        Option<LobbyStatsOptionsEndpointsGetByPersonalTypeReturnsMultipliersRangesParams>,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: String,
}
