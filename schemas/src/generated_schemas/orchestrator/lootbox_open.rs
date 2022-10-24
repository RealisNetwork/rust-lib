// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenParamsProductsParamsParams {
    #[serde(rename = "clientType")]
    pub client_type: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "isNft")]
    pub is_nft: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenParams {
    #[serde(rename = "products")]
    pub products: Vec<OrchestratorLootboxOpenParamsProductsParamsParams>,
    #[serde(rename = "lootboxIdentifier")]
    pub lootbox_identifier: String,
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for OrchestratorLootboxOpenParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"products\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"clientType\":{\"type\":\"string\",\"pattern\":\"^(none)|(hero)|(equipment)|(skin)|(lootbox)|(undistributedExperience)|(heroScroll)|(equipmentScroll)$\"},\"personalType\":{\"type\":\"string\"},\"isNft\":{\"type\":\"boolean\"}},\"required\":[\"personalType\",\"isNft\",\"clientType\"]}},\"lootboxIdentifier\":{\"type\":\"string\"},\"lootboxBindingId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"lootboxIdentifier\",\"lootboxBindingId\",\"products\"]}") . unwrap ()
    }
}
impl Agent for OrchestratorLootboxOpenParams {
    fn topic() -> &'static str {
        "orchestrator_lootbox_open"
    }
    fn method() -> &'static str {
        "lootbox_open"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenReturns(pub bool);
impl Schema for OrchestratorLootboxOpenReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for OrchestratorLootboxOpenReturns {
    fn topic() -> &'static str {
        "orchestrator_lootbox_open"
    }
    fn method() -> &'static str {
        "lootbox_open"
    }
    fn agent() -> &'static str {
        "orchestrator"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
