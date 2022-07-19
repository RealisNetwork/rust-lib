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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams {
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams
{
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams {
    #[serde(rename = "equipment")]
    pub equipment: i8,
    #[serde(rename = "itemUid")]
    pub item_uid: Option<i32>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "attributes")]
    pub attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsAttributesParams,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "equipmentAttributes")]
    pub equipment_attributes:
        ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsEquipmentAttributesParams,
    #[serde(rename = "experience")]
    pub experience: i32,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "toNextLevelExperience")]
    pub to_next_level_experience: i32,
    #[serde(rename = "slots")]
    pub slots:
        Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParamsSlotsParamsParams>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns(
    Vec<ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturnsParams>,
);
impl Schema for ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"active\":{\"type\":\"boolean\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"skillEffectPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"linkToExplorer\":{\"type\":\"string\"},\"isPending\":{\"type\":\"boolean\"},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"transactionHash\":{\"type\":\"string\"},\"equipmentAttributes\":{\"type\":\"object\",\"properties\":{\"moveSpeed\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"experience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelExperience\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slots\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"equipment\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"itemUid\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"equipment\"]}}},\"required\":[\"heroId\",\"level\",\"experience\",\"active\",\"bindingId\",\"linkToExplorer\",\"attributes\",\"equipmentAttributes\",\"slots\",\"isPending\",\"toNextLevelExperience\",\"toNextLevelScrolls\",\"maxLevel\",\"status\"]}}")
    }
}
