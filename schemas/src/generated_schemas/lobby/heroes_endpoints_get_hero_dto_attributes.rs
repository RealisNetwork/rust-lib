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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroDtoAttributesReturnsAttributesParams {
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsGetHeroDtoAttributesReturnsEquipmentAttributesParams {
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"attributes\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"strength\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\",\"mainCharacteristic\"]},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"attributes\",\"equipmentAttributes\"]}")
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
}
