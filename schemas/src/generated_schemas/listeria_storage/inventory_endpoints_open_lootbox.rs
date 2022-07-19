// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsOpenLootboxParams {
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
}
impl Schema for ListeriaStorageInventoryEndpointsOpenLootboxParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\"]}")
    }
}
impl<'de> Deserialize<'de> for ListeriaStorageInventoryEndpointsOpenLootboxReturns {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ListeriaStorageInventoryEndpointsOpenLootboxReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageInventoryEndpointsOpenLootboxReturns;
impl Schema for ListeriaStorageInventoryEndpointsOpenLootboxReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}