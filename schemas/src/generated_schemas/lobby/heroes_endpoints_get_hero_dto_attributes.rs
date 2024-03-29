// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroDtoAttributesParams {
    #[serde(rename = "heroId")]
    pub hero_id: i32,
}
impl Schema for LobbyHeroesEndpointsGetHeroDtoAttributesParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\"]}") . unwrap ()
    }
}
impl Agent for LobbyHeroesEndpointsGetHeroDtoAttributesParams {
    fn topic() -> &'static str {
        "lobby_heroesEndpoints_getHeroDTOAttributes"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getHeroDTOAttributes"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroDtoAttributesReturnsAttributesParams {
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroDtoAttributesReturnsEquipmentAttributesParams {
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "armor")]
    pub armor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroDtoAttributesReturns {
    #[serde(rename = "attributes")]
    pub attributes: LobbyHeroesEndpointsGetHeroDtoAttributesReturnsAttributesParams,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        LobbyHeroesEndpointsGetHeroDtoAttributesReturnsEquipmentAttributesParams,
}
impl Schema for LobbyHeroesEndpointsGetHeroDtoAttributesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"attributes\":{\"type\":\"object\",\"properties\":{\"ultEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"ultEffectPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"attributes\",\"equipmentAttributes\"]}")
    }
}
impl Agent for LobbyHeroesEndpointsGetHeroDtoAttributesReturns {
    fn topic() -> &'static str {
        "lobby_heroesEndpoints_getHeroDTOAttributes"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getHeroDTOAttributes"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
