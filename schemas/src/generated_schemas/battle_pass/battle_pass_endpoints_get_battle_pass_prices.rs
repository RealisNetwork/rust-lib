// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassPricesParams {}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassPricesParams {
    fn schema() -> Value {
        serde_json::from_str("{\"type\":\"object\",\"properties\":{}}").unwrap()
    }
}
impl Agent for BattlePassBattlePassEndpointsGetBattlePassPricesParams {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_getBattlePassPrices"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_getBattlePassPrices"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassPricesReturnsBattlePassPriceParamsParams {
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "battlePassType")]
    pub battle_pass_type: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassPricesReturns {
    #[serde(rename = "experiencePrice")]
    pub experience_price: String,
    #[serde(rename = "battlePassPrice")]
    pub battle_pass_price:
        Vec<BattlePassBattlePassEndpointsGetBattlePassPricesReturnsBattlePassPriceParamsParams>,
}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassPricesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"experiencePrice\":{\"type\":\"string\"},\"battlePassPrice\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"price\":{\"type\":\"string\"},\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"battlePassType\",\"price\"]}}},\"required\":[\"experiencePrice\",\"battlePassPrice\"]}")
    }
}
impl Agent for BattlePassBattlePassEndpointsGetBattlePassPricesReturns {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_getBattlePassPrices"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_getBattlePassPrices"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
}
