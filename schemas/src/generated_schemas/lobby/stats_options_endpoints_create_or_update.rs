// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsScrollsByLevelsParamsParams
{
    #[serde(rename = "scrollsCount")]
    pub scrolls_count: i64,
    #[serde(rename = "level")]
    pub level: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsAttributesPerLevelParams
{
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsExperienceCoefficientsParamsParams
{
    #[serde(rename = "level")]
    pub level: i64,
    #[serde(rename = "coefficient")]
    pub coefficient: i64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParams { # [serde (rename = "scrollsByLevels")] pub scrolls_by_levels : Vec < LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsScrollsByLevelsParamsParams > , # [serde (rename = "maxLevel")] pub max_level : i64 , # [serde (rename = "intelligence")] pub intelligence : i64 , # [serde (rename = "strength")] pub strength : i64 , # [serde (rename = "agility")] pub agility : i64 , # [serde (rename = "attributesPerLevel")] pub attributes_per_level : LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsAttributesPerLevelParams , # [serde (rename = "experience")] pub experience : i64 , # [serde (rename = "experienceCoefficients")] pub experience_coefficients : Vec < LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParamsExperienceCoefficientsParamsParams > , # [serde (rename = "baseExperience")] pub base_experience : i64 , # [serde (rename = "baseScrollsCount")] pub base_scrolls_count : i64 }
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsMultipliersRangesParams {
    #[serde(rename = "armor")]
    pub armor: Vec<i64>,
    #[serde(rename = "health")]
    pub health: Vec<i64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<i64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<i64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<i64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<i64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<i64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<i64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<i64>,
    #[serde(rename = "strength")]
    pub strength: Vec<i64>,
    #[serde(rename = "agility")]
    pub agility: Vec<i64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<i64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<i64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParamsAttributesRangesParams {
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<i64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<i64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<i64>,
    #[serde(rename = "agility")]
    pub agility: Vec<i64>,
    #[serde(rename = "health")]
    pub health: Vec<i64>,
    #[serde(rename = "strength")]
    pub strength: Vec<i64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<i64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<i64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<i64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<i64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<i64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<i64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<i64>,
    #[serde(rename = "armor")]
    pub armor: Vec<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyStatsOptionsEndpointsCreateOrUpdateParams {
    #[serde(rename = "levelUpOptions")]
    pub level_up_options:
        Option<LobbyStatsOptionsEndpointsCreateOrUpdateParamsLevelUpOptionsParams>,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: Option<String>,
    #[serde(rename = "multipliersRanges")]
    pub multipliers_ranges:
        Option<LobbyStatsOptionsEndpointsCreateOrUpdateParamsMultipliersRangesParams>,
    #[serde(rename = "attributesRanges")]
    pub attributes_ranges:
        Option<LobbyStatsOptionsEndpointsCreateOrUpdateParamsAttributesRangesParams>,
}
pub type LobbyStatsOptionsEndpointsCreateOrUpdateReturns = bool;
