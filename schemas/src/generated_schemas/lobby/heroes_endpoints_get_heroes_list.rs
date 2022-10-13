// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyHeroesEndpointsGetHeroesListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyHeroesEndpointsGetHeroesListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyHeroesEndpointsGetHeroesListParams;
impl Schema for LobbyHeroesEndpointsGetHeroesListParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyHeroesEndpointsGetHeroesListParams {
    fn topic() -> &'static str {
        "lobby_heroesEndpoints_getHeroesList"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getHeroesList"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroesListReturnsParamsStatsRangesParams {
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "attackDamage")]
    pub attack_damage: Vec<String>,
    #[serde(rename = "health")]
    pub health: Vec<String>,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: Vec<String>,
    #[serde(rename = "armor")]
    pub armor: Vec<String>,
    #[serde(rename = "ultPower")]
    pub ult_power: Vec<String>,
    #[serde(rename = "skillPower")]
    pub skill_power: Vec<String>,
    #[serde(rename = "agility")]
    pub agility: Vec<String>,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: Vec<String>,
    #[serde(rename = "strength")]
    pub strength: Vec<String>,
    #[serde(rename = "intelligence")]
    pub intelligence: Vec<String>,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: Vec<String>,
    #[serde(rename = "moveSpeed")]
    pub move_speed: Vec<String>,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: Vec<String>,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroesListReturnsParams {
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "statsRanges")]
    pub stats_ranges: LobbyHeroesEndpointsGetHeroesListReturnsParamsStatsRangesParams,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroesListReturns(
    pub Vec<LobbyHeroesEndpointsGetHeroesListReturnsParams>,
);
impl Schema for LobbyHeroesEndpointsGetHeroesListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"statsRanges\":{\"type\":\"object\",\"properties\":{\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"attackDamage\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"health\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"vampirismPower\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"armor\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"ultPower\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"skillPower\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"agility\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"skillEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"strength\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"intelligence\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"ultEffectPower\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"moveSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"attackReloadSpeed\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}},\"healthRegenPercent\":{\"type\":\"array\",\"items\":{\"type\":\"string\"}}},\"required\":[\"mainCharacteristic\",\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"heroId\",\"statsRanges\"]}}")
    }
}
impl Agent for LobbyHeroesEndpointsGetHeroesListReturns {
    fn topic() -> &'static str {
        "lobby_heroesEndpoints_getHeroesList"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getHeroesList"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
