// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(ListeriaStorageHeroesEndpointsHeroAddedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationParams;
impl Schema for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for ListeriaStorageHeroesEndpointsHeroAddedNotificationParams {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_heroAddedNotification"
    }
    fn method() -> &'static str {
        "heroesEndpoints_heroAddedNotification"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams {
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "agility")]
    pub agility: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsAttributesParams {
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams {
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
    #[serde(rename = "equipment")]
    pub equipment: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns {
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsEquipmentAttributesParams,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "attributes")]
    pub attributes: ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsAttributesParams,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "slots")]
    pub slots: Vec<ListeriaStorageHeroesEndpointsHeroAddedNotificationReturnsSlotsParamsParams>,
}
impl Schema for ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"skillPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"healthRegenPercent\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"active\":{\"type\":\"boolean\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"transactionHash\":{\"type\":\"string\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isPending\":{\"type\":\"boolean\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"attackReloadSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"blockId\":{\"type\":\"string\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}")
    }
}
impl Agent for ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_heroAddedNotification"
    }
    fn method() -> &'static str {
        "heroesEndpoints_heroAddedNotification"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
}
