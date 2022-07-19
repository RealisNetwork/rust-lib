// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenParamsProductsParamsParams {
    #[serde(rename = "isNft")]
    pub is_nft: bool,
    #[serde(rename = "clientType")]
    pub client_type: String,
    #[serde(rename = "personalType")]
    pub personal_type: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenParams {
    #[serde(rename = "products")]
    pub products: Vec<OrchestratorLootboxOpenParamsProductsParamsParams>,
    #[serde(rename = "lootboxIdentifier")]
    pub lootbox_identifier: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: i64,
}
impl Schema for OrchestratorLootboxOpenParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"products\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"isNft\":{\"type\":\"boolean\"},\"clientType\":{\"type\":\"string\",\"pattern\":\"^(none)|(hero)|(equipment)|(skin)|(lootbox)|(undistributedExperience)|(heroScroll)|(equipmentScroll)$\"},\"personalType\":{\"type\":\"string\"}},\"required\":[\"personalType\",\"isNft\",\"clientType\"]}},\"lootboxIdentifier\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"lootboxBindingId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"lootboxIdentifier\",\"lootboxBindingId\",\"products\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenReturns(bool);
impl Schema for OrchestratorLootboxOpenReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
