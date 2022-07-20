// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParamsAttributesCoefficientsParams
{
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: i64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: i64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: i64,
    #[serde(rename = "agility")]
    pub agility: i64,
    #[serde(rename = "ultPower")]
    pub ult_power: i64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: i64,
    #[serde(rename = "intelligence")]
    pub intelligence: i64,
    #[serde(rename = "armor")]
    pub armor: i64,
    #[serde(rename = "strength")]
    pub strength: i64,
    #[serde(rename = "health")]
    pub health: i64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: i64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: i64,
    #[serde(rename = "skillPower")]
    pub skill_power: i64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams { # [serde (rename = "attributesCoefficients")] pub attributes_coefficients : MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParamsAttributesCoefficientsParams , # [serde (rename = "level")] pub level : i32 }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParams {
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams,
}
impl Schema for MarketPlaceMarketPlaceAddToMarketPlaceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{\"attributesCoefficients\":{\"type\":\"object\",\"properties\":{\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]},\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"level\",\"attributesCoefficients\"]}},\"required\":[\"userId\",\"price\",\"category\",\"personalType\",\"productId\",\"additionalParams\"]}")
    }
}
impl Agent for MarketPlaceMarketPlaceAddToMarketPlaceParams {
    fn topic() -> &'static str {
        "market-place_marketPlace_addToMarketPlace"
    }
    fn method() -> &'static str {
        "marketPlace_addToMarketPlace"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceReturns(bool);
impl Schema for MarketPlaceMarketPlaceAddToMarketPlaceReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for MarketPlaceMarketPlaceAddToMarketPlaceReturns {
    fn topic() -> &'static str {
        "market-place_marketPlace_addToMarketPlace"
    }
    fn method() -> &'static str {
        "marketPlace_addToMarketPlace"
    }
    fn agent() -> &'static str {
        "market-place"
    }
}
