// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams;
impl Schema for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams {
    fn schema() -> Value {
        serde_json::json!("{}")
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
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams
{
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
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
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "strength")]
    pub strength: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams {
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "slots")]
    pub slots:
        Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams>,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "attributes")]
    pub attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "status")]
    pub status: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns(
    pub Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams>,
);
impl Schema for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"moveSpeed\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"armor\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"blockId\":{\"type\":\"string\"},\"isPending\":{\"type\":\"boolean\"},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"equipment\"]}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"healthRegenPercent\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"health\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"active\":{\"type\":\"boolean\"},\"transactionHash\":{\"type\":\"string\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}}")
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
