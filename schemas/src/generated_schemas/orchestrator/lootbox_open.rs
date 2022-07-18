// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenParamsProductsParamsParams {
    #[serde(rename = "personalType")]
    pub personal_type: String,
    #[serde(rename = "clientType")]
    pub client_type: String,
    #[serde(rename = "isNft")]
    pub is_nft: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestratorLootboxOpenParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "products")]
    pub products: Vec<OrchestratorLootboxOpenParamsProductsParamsParams>,
    #[serde(rename = "lootboxBindingId")]
    pub lootbox_binding_id: i64,
    #[serde(rename = "lootboxIdentifier")]
    pub lootbox_identifier: String,
}
impl Schema for OrchestratorLootboxOpenParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type OrchestratorLootboxOpenReturns = bool;
