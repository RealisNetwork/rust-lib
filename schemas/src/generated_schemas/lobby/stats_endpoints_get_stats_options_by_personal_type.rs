// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
impl Schema for LobbyStatsEndpointsGetStatsOptionsByPersonalTypeParams {
    fn schema() -> Value {
        todo!()
    }
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
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams
{
    #[serde(rename = "coefficient")]
    pub coefficient: String,
    #[serde(rename = "level")]
    pub level: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams
{
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParams { # [serde (rename = "intelligence")] pub intelligence : String , # [serde (rename = "scrollsByLevels")] pub scrolls_by_levels : Vec < LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsScrollsByLevelsParamsParams > , # [serde (rename = "experienceCoefficients")] pub experience_coefficients : Vec < LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsExperienceCoefficientsParamsParams > , # [serde (rename = "attributesPerLevel")] pub attributes_per_level : LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParamsAttributesPerLevelParams , # [serde (rename = "baseExperience")] pub base_experience : i32 , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "maxLevel")] pub max_level : i32 , # [serde (rename = "agility")] pub agility : String , # [serde (rename = "strength")] pub strength : String , # [serde (rename = "baseScrollsCount")] pub base_scrolls_count : i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsStatsRangesParams {
    #[serde(rename = "agility")]
    pub agility: Vec<i64>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<i64>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<i64>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<i64>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<i64>,
    #[serde(rename = "health")]
    pub health: Vec<i64>,
    #[serde(rename = "armor")]
    pub armor: Vec<i64>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<i64>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<i64>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<i64>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<i64>,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<i64>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<i64>,
    #[serde(rename = "strength")]
    pub strength: Vec<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturns {
    #[serde(rename = "levelUpOptions")]
    pub level_up_options:
        LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsLevelUpOptionsParams,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: String,
    #[serde(rename = "statsRanges")]
    pub stats_ranges: LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturnsStatsRangesParams,
}
