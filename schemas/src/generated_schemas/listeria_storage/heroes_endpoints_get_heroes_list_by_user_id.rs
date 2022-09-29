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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams {
    #[serde(rename = "equipment")]
    pub equipment: i8,
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams
{
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "armor")]
    pub armor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams {
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams {
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "slots")]
    pub slots:
        Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams>,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "attributes")]
    pub attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns(
    pub Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams>,
);
impl Schema for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"equipment\"]}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"active\":{\"type\":\"boolean\"},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"vampirismPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"health\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"transactionHash\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"attackReloadSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"agility\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"isPending\":{\"type\":\"boolean\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}}")
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
}
