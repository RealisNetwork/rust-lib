// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassPricesParams {}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassPricesParams {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{},\"required\":null}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassPricesReturnsBattlePassPriceParamsParams {
    #[serde(rename = "battlePassType")]
    pub battle_pass_type: i32,
    #[serde(rename = "price")]
    pub price: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsGetBattlePassPricesReturns {
    #[serde(rename = "battlePassPrice")]
    pub battle_pass_price:
        Vec<BattlePassBattlePassEndpointsGetBattlePassPricesReturnsBattlePassPriceParamsParams>,
    #[serde(rename = "experiencePrice")]
    pub experience_price: String,
}
impl Schema for BattlePassBattlePassEndpointsGetBattlePassPricesReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"battlePassPrice\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"price\":{\"type\":\"string\"}},\"required\":[\"battlePassType\",\"price\"]}},\"experiencePrice\":{\"type\":\"string\"}},\"required\":[\"experiencePrice\",\"battlePassPrice\"]}")
    }
}
