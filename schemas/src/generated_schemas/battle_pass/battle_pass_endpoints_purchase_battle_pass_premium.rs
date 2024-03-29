// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsPurchaseBattlePassPremiumParams {
    #[serde(rename = "battlePassType")]
    pub battle_pass_type: i32,
}
impl Schema for BattlePassBattlePassEndpointsPurchaseBattlePassPremiumParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"battlePassType\"]}") . unwrap ()
    }
}
impl Agent for BattlePassBattlePassEndpointsPurchaseBattlePassPremiumParams {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_purchaseBattlePassPremium"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_purchaseBattlePassPremium"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattlePassBattlePassEndpointsPurchaseBattlePassPremiumReturns {
    #[serde(rename = "battlePassType")]
    pub battle_pass_type: i32,
}
impl Schema for BattlePassBattlePassEndpointsPurchaseBattlePassPremiumReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"battlePassType\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"battlePassType\"]}")
    }
}
impl Agent for BattlePassBattlePassEndpointsPurchaseBattlePassPremiumReturns {
    fn topic() -> &'static str {
        "battle-pass_battlePassEndpoints_purchaseBattlePassPremium"
    }
    fn method() -> &'static str {
        "battlePassEndpoints_purchaseBattlePassPremium"
    }
    fn agent() -> &'static str {
        "battle-pass"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
