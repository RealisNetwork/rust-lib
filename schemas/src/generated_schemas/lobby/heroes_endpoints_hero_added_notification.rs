// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyHeroesEndpointsHeroAddedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(LobbyHeroesEndpointsHeroAddedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationParams;
impl Schema for LobbyHeroesEndpointsHeroAddedNotificationParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for LobbyHeroesEndpointsHeroAddedNotificationParams {
    fn topic() -> &'static str {
        "lobby_heroesEndpoints_heroAddedNotification"
    }
    fn method() -> &'static str {
        "heroesEndpoints_heroAddedNotification"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsAttributesParams {
    #[serde(rename = "armor", deserialize_with = "deserialize_to_string")]
    pub armor: String,
    #[serde(rename = "ultEffectPower", deserialize_with = "deserialize_to_string")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower", deserialize_with = "deserialize_to_string")]
    pub vampirism_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "agility", deserialize_with = "deserialize_to_string")]
    pub agility: String,
    #[serde(
        rename = "attackReloadSpeed",
        deserialize_with = "deserialize_to_string"
    )]
    pub attack_reload_speed: String,
    #[serde(rename = "ultPower", deserialize_with = "deserialize_to_string")]
    pub ult_power: String,
    #[serde(rename = "skillPower", deserialize_with = "deserialize_to_string")]
    pub skill_power: String,
    #[serde(rename = "moveSpeed", deserialize_with = "deserialize_to_string")]
    pub move_speed: String,
    #[serde(rename = "attackDamage", deserialize_with = "deserialize_to_string")]
    pub attack_damage: String,
    #[serde(
        rename = "healthRegenPercent",
        deserialize_with = "deserialize_to_string"
    )]
    pub health_regen_percent: String,
    #[serde(
        rename = "skillEffectPower",
        deserialize_with = "deserialize_to_string"
    )]
    pub skill_effect_power: String,
    #[serde(rename = "health", deserialize_with = "deserialize_to_string")]
    pub health: String,
    #[serde(rename = "strength", deserialize_with = "deserialize_to_string")]
    pub strength: String,
    #[serde(rename = "intelligence", deserialize_with = "deserialize_to_string")]
    pub intelligence: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams {
    #[serde(
        rename = "healthRegenPercent",
        deserialize_with = "deserialize_to_string"
    )]
    pub health_regen_percent: String,
    #[serde(
        rename = "attackReloadSpeed",
        deserialize_with = "deserialize_to_string"
    )]
    pub attack_reload_speed: String,
    #[serde(rename = "ultEffectPower", deserialize_with = "deserialize_to_string")]
    pub ult_effect_power: String,
    #[serde(rename = "moveSpeed", deserialize_with = "deserialize_to_string")]
    pub move_speed: String,
    #[serde(rename = "ultPower", deserialize_with = "deserialize_to_string")]
    pub ult_power: String,
    #[serde(rename = "skillPower", deserialize_with = "deserialize_to_string")]
    pub skill_power: String,
    #[serde(rename = "intelligence", deserialize_with = "deserialize_to_string")]
    pub intelligence: String,
    #[serde(rename = "health", deserialize_with = "deserialize_to_string")]
    pub health: String,
    #[serde(rename = "attackDamage", deserialize_with = "deserialize_to_string")]
    pub attack_damage: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "vampirismPower", deserialize_with = "deserialize_to_string")]
    pub vampirism_power: String,
    #[serde(rename = "strength", deserialize_with = "deserialize_to_string")]
    pub strength: String,
    #[serde(
        rename = "skillEffectPower",
        deserialize_with = "deserialize_to_string"
    )]
    pub skill_effect_power: String,
    #[serde(rename = "agility", deserialize_with = "deserialize_to_string")]
    pub agility: String,
    #[serde(rename = "armor", deserialize_with = "deserialize_to_string")]
    pub armor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams {
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
    #[serde(rename = "equipment")]
    pub equipment: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturns {
    #[serde(rename = "attributes")]
    pub attributes: LobbyHeroesEndpointsHeroAddedNotificationReturnsAttributesParams,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        LobbyHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "slots")]
    pub slots: Vec<LobbyHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams>,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "linkToExplorer", deserialize_with = "deserialize_to_string")]
    pub link_to_explorer: String,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
}
impl Schema for LobbyHeroesEndpointsHeroAddedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"attributes\":{\"type\":\"object\",\"properties\":{\"armor\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"agility\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"healthRegenPercent\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"vampirismPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"active\":{\"type\":\"boolean\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isPending\":{\"type\":\"boolean\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"transactionHash\":{\"type\":\"string\"},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}")
    }
}
impl Agent for LobbyHeroesEndpointsHeroAddedNotificationReturns {
    fn topic() -> &'static str {
        "lobby_heroesEndpoints_heroAddedNotification"
    }
    fn method() -> &'static str {
        "heroesEndpoints_heroAddedNotification"
    }
    fn agent() -> &'static str {
        "lobby"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
