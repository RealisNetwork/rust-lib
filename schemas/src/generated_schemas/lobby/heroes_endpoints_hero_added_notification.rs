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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams {
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsAttributesParams {
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
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
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        LobbyHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams,
    #[serde(rename = "attributes")]
    pub attributes: LobbyHeroesEndpointsHeroAddedNotificationReturnsAttributesParams,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "slots")]
    pub slots: Vec<LobbyHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams>,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
}
impl Schema for LobbyHeroesEndpointsHeroAddedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"blockId\":{\"type\":\"string\"},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"agility\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"skillEffectPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"attributes\":{\"type\":\"object\",\"properties\":{\"strength\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"agility\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"active\":{\"type\":\"boolean\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"transactionHash\":{\"type\":\"string\"},\"linkToExplorer\":{\"type\":\"string\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isPending\":{\"type\":\"boolean\"},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}")
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
}
