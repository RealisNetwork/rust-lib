// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsLobbyPurchaseValidationParams {
    #[serde(rename = "storeId")]
    pub store_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    #[serde(rename = "purchaseToken")]
    pub purchase_token: String,
}
impl Schema for CatsLobbyPurchaseValidationParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"storeId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"productId\":{\"type\":\"string\"},\"purchaseToken\":{\"type\":\"string\"}},\"required\":[\"userId\",\"storeId\",\"productId\",\"purchaseToken\"]}")
    }
}
impl Agent for CatsLobbyPurchaseValidationParams {
    fn topic() -> &'static str {
        "cats_lobby_purchaseValidation"
    }
    fn method() -> &'static str {
        "lobby_purchaseValidation"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
impl<'de> Deserialize<'de> for CatsLobbyPurchaseValidationReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(CatsLobbyPurchaseValidationReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct CatsLobbyPurchaseValidationReturns;
impl Schema for CatsLobbyPurchaseValidationReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for CatsLobbyPurchaseValidationReturns {
    fn topic() -> &'static str {
        "cats_lobby_purchaseValidation"
    }
    fn method() -> &'static str {
        "lobby_purchaseValidation"
    }
    fn agent() -> &'static str {
        "cats"
    }
}
