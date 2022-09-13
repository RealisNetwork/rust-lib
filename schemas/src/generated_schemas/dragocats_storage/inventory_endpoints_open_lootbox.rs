// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsOpenLootboxParams {
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
}
impl Schema for DragocatsStorageInventoryEndpointsOpenLootboxParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"bindingId\"]}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsOpenLootboxParams {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_openLootbox"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_openLootbox"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
}
impl<'de> Deserialize<'de> for DragocatsStorageInventoryEndpointsOpenLootboxReturns {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragocatsStorageInventoryEndpointsOpenLootboxReturns)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragocatsStorageInventoryEndpointsOpenLootboxReturns;
impl Schema for DragocatsStorageInventoryEndpointsOpenLootboxReturns {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsOpenLootboxReturns {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_openLootbox"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_openLootbox"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
}
