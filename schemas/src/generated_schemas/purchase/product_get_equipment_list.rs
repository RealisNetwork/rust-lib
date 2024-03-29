// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for PurchaseProductGetEquipmentListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(PurchaseProductGetEquipmentListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct PurchaseProductGetEquipmentListParams;
impl Schema for PurchaseProductGetEquipmentListParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for PurchaseProductGetEquipmentListParams {
    fn topic() -> &'static str {
        "purchase_product_getEquipmentList"
    }
    fn method() -> &'static str {
        "product_getEquipmentList"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseProductGetEquipmentListReturnsParamsStatsParams {
    #[serde(rename = "attackDamage")]
    pub attack_damage: i32,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i32,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "agility")]
    pub agility: i32,
    #[serde(rename = "intelligence")]
    pub intelligence: i32,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "skillPower")]
    pub skill_power: i32,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i32,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
    #[serde(rename = "strength")]
    pub strength: i32,
    #[serde(rename = "ultPower")]
    pub ult_power: i32,
    #[serde(rename = "health")]
    pub health: i32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseProductGetEquipmentListReturnsParams {
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "stats")]
    pub stats: PurchaseProductGetEquipmentListReturnsParamsStatsParams,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "equipmentId")]
    pub equipment_id: i32,
    #[serde(rename = "productType")]
    pub product_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseProductGetEquipmentListReturns(
    pub Vec<PurchaseProductGetEquipmentListReturnsParams>,
);
impl Schema for PurchaseProductGetEquipmentListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)|(TLIS)$\"},\"stats\":{\"type\":\"object\",\"properties\":{\"attackDamage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"armor\":{\"type\":\"string\"},\"agility\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attackReloadSpeed\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"vampirismPower\":{\"type\":\"string\"},\"healthRegenPercent\":{\"type\":\"string\"},\"strength\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"health\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"moveSpeed\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"price\":{\"type\":\"string\"},\"equipmentId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productType\":{\"type\":\"string\"}},\"required\":[\"equipmentId\",\"productType\",\"currency\",\"price\",\"stats\"]}}")
    }
}
impl Agent for PurchaseProductGetEquipmentListReturns {
    fn topic() -> &'static str {
        "purchase_product_getEquipmentList"
    }
    fn method() -> &'static str {
        "product_getEquipmentList"
    }
    fn agent() -> &'static str {
        "purchase"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
