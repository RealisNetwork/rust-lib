// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams;
impl Schema for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_getHeroesListByUserId"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getHeroesListByUserId"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams {
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
    #[serde(rename = "equipment")]
    pub equipment: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams {
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams
{
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "health")]
    pub health: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams {
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "slots")]
    pub slots:
        Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams>,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "attributes")]
    pub attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "status")]
    pub status: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns(
    pub Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams>,
);
impl Schema for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"linkToExplorer\":{\"type\":\"string\"},\"transactionHash\":{\"type\":\"string\"},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"active\":{\"type\":\"boolean\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isPending\":{\"type\":\"boolean\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}},\"blockId\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"vampirismPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"ultEffectPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"strength\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}}")
    }
}
impl Agent for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns {
    fn topic() -> &'static str {
        "listeria-storage_heroesEndpoints_getHeroesListByUserId"
    }
    fn method() -> &'static str {
        "heroesEndpoints_getHeroesListByUserId"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
