// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsGetItemByItemIdAndUserIdParams {
    #[serde(rename = "itemId")]
    pub item_id: i32,
}
impl Schema for LobbyEquipmentEndpointsGetItemByItemIdAndUserIdParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"itemId\"]}")
    }
}
impl Agent for LobbyEquipmentEndpointsGetItemByItemIdAndUserIdParams {
    fn topic() -> &'static str {
        "lobby_equipmentEndpoints_getItemByItemIdAndUserId"
    }
    fn method() -> &'static str {
        "equipmentEndpoints_getItemByItemIdAndUserId"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsAttributesParams {
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsMultipliersParams {
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "armor")]
    pub armor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturns {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "attributes")]
    pub attributes: LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsAttributesParams,
    #[serde(rename = "multipliers")]
    pub multipliers: LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsMultipliersParams,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "slotId")]
    pub slot_id: i32,
}
impl Schema for LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"},\"attributes\":{\"type\":\"object\",\"properties\":{\"armor\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"multipliers\":{\"type\":\"object\",\"properties\":{\"agility\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"userId\":{\"type\":\"string\"},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"type\",\"slotId\",\"userId\",\"itemId\",\"level\",\"attributes\",\"multipliers\"]}")
    }
}
impl Agent for LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturns {
    fn topic() -> &'static str {
        "lobby_equipmentEndpoints_getItemByItemIdAndUserId"
    }
    fn method() -> &'static str {
        "equipmentEndpoints_getItemByItemIdAndUserId"
    }
    fn agent() -> &'static str {
        "lobby"
    }
}
