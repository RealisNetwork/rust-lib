// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParamsAttributesCoefficientsParams
{
    #[serde(rename = "health")]
    pub health: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "skillPower")]
    pub skill_power: f64,
    #[serde(rename = "skillEffectPower")]
    pub skill_effect_power: f64,
    #[serde(rename = "armor")]
    pub armor: f64,
    #[serde(rename = "ultEffectPower")]
    pub ult_effect_power: f64,
    #[serde(rename = "ultPower")]
    pub ult_power: f64,
    #[serde(rename = "strength")]
    pub strength: f64,
    #[serde(rename = "vampirismPower")]
    pub vampirism_power: f64,
    #[serde(rename = "agility")]
    pub agility: f64,
    #[serde(rename = "healthRegenPercent")]
    pub health_regen_percent: f64,
    #[serde(rename = "attackReloadSpeed")]
    pub attack_reload_speed: f64,
    #[serde(rename = "intelligence")]
    pub intelligence: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams { # [serde (rename = "level")] pub level : i32 , # [serde (rename = "attributesCoefficients")] pub attributes_coefficients : MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParamsAttributesCoefficientsParams }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPlaceMarketPlaceAddToMarketPlaceParams {
    #[serde(rename = "price")]
    pub price: i32,
    #[serde(rename = "category")]
    pub category: i32,
    #[serde(rename = "additionalParams")]
    pub additional_params: MarketPlaceMarketPlaceAddToMarketPlaceParamsAdditionalParamsParams,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
impl Schema for MarketPlaceMarketPlaceAddToMarketPlaceParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"additionalParams\":{\"type\":\"object\",\"properties\":{\"level\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"attributesCoefficients\":{\"type\":\"object\",\"properties\":{\"health\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"moveSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackDamage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"skillEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"armor\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultEffectPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"ultPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"strength\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"vampirismPower\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"agility\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"healthRegenPercent\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"attackReloadSpeed\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"intelligence\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"strength\",\"agility\",\"intelligence\",\"health\",\"healthRegenPercent\",\"armor\",\"moveSpeed\",\"attackDamage\",\"attackReloadSpeed\",\"skillPower\",\"skillEffectPower\",\"ultPower\",\"ultEffectPower\",\"vampirismPower\"]}},\"required\":[\"level\",\"attributesCoefficients\"]},\"userId\":{\"type\":\"string\"},\"personalType\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"userId\",\"price\",\"category\",\"personalType\",\"productId\",\"additionalParams\"]}")
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
pub struct MarketPlaceMarketPlaceAddToMarketPlaceReturns(pub bool);
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
