// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for GameBalancerCoefficientEndpointsGetParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(GameBalancerCoefficientEndpointsGetParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct GameBalancerCoefficientEndpointsGetParams;
impl Schema for GameBalancerCoefficientEndpointsGetParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameBalancerCoefficientEndpointsGetReturns {
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
}
