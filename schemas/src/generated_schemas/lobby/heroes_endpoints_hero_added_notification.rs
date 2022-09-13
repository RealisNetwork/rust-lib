// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyHeroesEndpointsHeroAddedNotificationParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LobbyHeroesEndpointsHeroAddedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationParams;
impl Schema for LobbyHeroesEndpointsHeroAddedNotificationParams {
    fn schema() -> Value {
        serde_json::json!("{}")
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
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams {
    #[serde(rename = "equipment")]
    pub equipment: i8,
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsAttributesParams {
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "health")]
    pub health: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams {
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyHeroesEndpointsHeroAddedNotificationReturns {
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "slots")]
    pub slots: Vec<LobbyHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams>,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "attributes")]
    pub attributes: LobbyHeroesEndpointsHeroAddedNotificationReturnsAttributesParams,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        LobbyHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams,
}
impl Schema for LobbyHeroesEndpointsHeroAddedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"transactionHash\":{\"type\":\"string\"},\"linkToExplorer\":{\"type\":\"string\"},\"isPending\":{\"type\":\"boolean\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"active\":{\"type\":\"boolean\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"equipment\"]}},\"blockId\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"ultEffectPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"intelligence\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"attackReloadSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"health\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}")
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
