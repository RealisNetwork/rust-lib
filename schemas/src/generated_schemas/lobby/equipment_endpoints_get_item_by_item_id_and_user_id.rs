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
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"itemId\"]}") . unwrap ()
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
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "health")]
    pub health: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsMultipliersParams {
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "attackDamage")]
    pub attack_damage: String,
    #[serde(rename = "strength")]
    pub strength: String,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: String,
    #[serde(rename = "ultPower")]
    pub ult_power: String,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: String,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "health")]
    pub health: String,
    #[serde(rename = "intelligence")]
    pub intelligence: String,
    #[serde(rename = "skillPower")]
    pub skill_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "agility")]
    pub agility: String,
    #[serde(rename = "armor")]
    pub armor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturns {
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "level")]
    pub level: i32,
    #[serde(rename = "attributes")]
    pub attributes: LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsAttributesParams,
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "multipliers")]
    pub multipliers: LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturnsMultipliersParams,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attributes\":{\"type\":\"object\",\"properties\":{\"agility\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"moveSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"multipliers\":{\"type\":\"object\",\"properties\":{\"moveSpeed\":{\"type\":\"string\"},\"attackDamage\":{\"type\":\"string\"},\"strength\":{\"type\":\"string\"},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillEffectPower\":{\"type\":\"string\"},\"ultPower\":{\"type\":\"string\"},\"ultEffectPower\":{\"type\":\"string\"},\"vampirismPower\":{\"type\":\"string\"},\"health\":{\"type\":\"string\"},\"intelligence\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"agility\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"type\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"type\",\"slotId\",\"userId\",\"itemId\",\"level\",\"attributes\",\"multipliers\"]}")
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
