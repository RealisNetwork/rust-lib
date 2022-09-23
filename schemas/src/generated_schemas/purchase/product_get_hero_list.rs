// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for PurchaseProductGetHeroListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(PurchaseProductGetHeroListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct PurchaseProductGetHeroListParams;
impl Schema for PurchaseProductGetHeroListParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for PurchaseProductGetHeroListParams {
    fn topic() -> &'static str {
        "purchase_product_getHeroList"
    }
    fn method() -> &'static str {
        "product_getHeroList"
    }
    fn agent() -> &'static str {
        "purchase"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseProductGetHeroListReturnsParamsStatsParams {
    #[serde(rename = "health")]
    pub health: i32,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i32,
    #[serde(rename = "moveSpeed")]
    pub move_speed: String,
    #[serde(rename = "armor")]
    pub armor: String,
    #[serde(rename = "skillPower")]
    pub skill_power: i32,
    #[serde(rename = "strength")]
    pub strength: i32,
    #[serde(rename = "intelligence")]
    pub intelligence: i32,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: String,
    #[serde(rename = "mainCharacteristic")]
    pub main_characteristic: i8,
    #[serde(rename = "ultPower")]
    pub ult_power: i32,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i32,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i32,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: String,
    #[serde(rename = "agility")]
    pub agility: i32,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseProductGetHeroListReturnsParams {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "heroId")]
    pub hero_id: i8,
    #[serde(rename = "stats")]
    pub stats: PurchaseProductGetHeroListReturnsParamsStatsParams,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "productType")]
    pub product_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseProductGetHeroListReturns(pub Vec<PurchaseProductGetHeroListReturnsParams>);
impl Schema for PurchaseProductGetHeroListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"string\"},\"heroId\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"stats\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"moveSpeed\":{\"type\":\"string\"},\"armor\":{\"type\":\"string\"},\"skillPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attackReloadSpeed\":{\"type\":\"string\"},\"mainCharacteristic\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"vampirismPower\":{\"type\":\"string\"},\"agility\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"healthRegenPercent\":{\"type\":\"string\"}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"mainCharacteristic\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"currency\":{\"type\":\"string\",\"pattern\":\"^(ETH)|(LIS)|(WLIS)$\"},\"productType\":{\"type\":\"string\"}},\"required\":[\"heroId\",\"productType\",\"currency\",\"price\",\"stats\"]}}")
    }
}
impl Agent for PurchaseProductGetHeroListReturns {
    fn topic() -> &'static str {
        "purchase_product_getHeroList"
    }
    fn method() -> &'static str {
        "product_getHeroList"
    }
    fn agent() -> &'static str {
        "purchase"
    }
}
