// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageHeroesEndpointsStatsUpdatedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(ListeriaStorageHeroesEndpointsStatsUpdatedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageHeroesEndpointsStatsUpdatedNotificationParams;
impl Schema for ListeriaStorageHeroesEndpointsStatsUpdatedNotificationParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for ListeriaStorageHeroesEndpointsStatsUpdatedNotificationParams {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_statsUpdatedNotification"
    }
    fn method() -> &'static str {
        "heroesEndpoints_statsUpdatedNotification"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParamsEquipmentAttributesParams
{
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParamsSlotsParamsParams
{
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
    #[serde(rename = "equipment")]
    pub equipment: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParamsAttributesParams {
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParams { # [serde (rename = "transactionHash")] pub transaction_hash : Option < String > , # [serde (rename = "level")] pub level : i16 , # [serde (rename = "isPending")] pub is_pending : bool , # [serde (rename = "status")] pub status : i32 , # [serde (rename = "toNextLevelExperience")] pub to_next_level_experience : i32 , # [serde (rename = "heroId")] pub hero_id : i8 , # [serde (rename = "linkToExplorer")] pub link_to_explorer : String , # [serde (rename = "equipmentAttributes")] pub equipment_attributes : ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParamsEquipmentAttributesParams , # [serde (rename = "slots")] pub slots : Vec < ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParamsSlotsParamsParams > , # [serde (rename = "toNextLevelScrolls")] pub to_next_level_scrolls : i32 , # [serde (rename = "blockId")] pub block_id : Option < String > , # [serde (rename = "experience")] pub experience : i32 , # [serde (rename = "maxLevel")] pub max_level : i16 , # [serde (rename = "bindingId")] pub binding_id : i32 , # [serde (rename = "attributes")] pub attributes : ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParamsAttributesParams , # [serde (rename = "active")] pub active : bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturns {
    #[serde(rename = "reason")]
    pub reason: i8,
    #[serde(rename = "hero")]
    pub hero: ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturnsHeroParams,
}
impl Schema for ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"reason\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"hero\":{\"type\":\"object\",\"properties\":{\"transactionHash\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"isPending\":{\"type\":\"boolean\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"linkToExplorer\":{\"type\":\"string\"},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"strength\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"intelligence\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"active\":{\"type\":\"boolean\"}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}},\"required\":[\"hero\",\"reason\"]}")
    }
}
impl Agent for ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturns {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_statsUpdatedNotification"
    }
    fn method() -> &'static str {
        "heroesEndpoints_statsUpdatedNotification"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
