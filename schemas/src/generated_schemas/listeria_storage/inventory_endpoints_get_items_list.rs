// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageInventoryEndpointsGetItemsListParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(ListeriaStorageInventoryEndpointsGetItemsListParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageInventoryEndpointsGetItemsListParams;
impl Schema for ListeriaStorageInventoryEndpointsGetItemsListParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
impl Agent for ListeriaStorageInventoryEndpointsGetItemsListParams {
    fn topic() -> &'static str {
        "listeria-storage_inventoryEndpoints_getItemsList"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getItemsList"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetItemsListReturnsParamsEffectsParamsParams {
    #[serde(rename = "statName")]
    pub stat_name: i8,
    #[serde(rename = "power")]
    pub power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetItemsListReturnsParams {
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "effects")]
    pub effects: Vec<ListeriaStorageInventoryEndpointsGetItemsListReturnsParamsEffectsParamsParams>,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "equipmentItemId")]
    pub equipment_item_id: i32,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetItemsListReturns(
    pub Vec<ListeriaStorageInventoryEndpointsGetItemsListReturnsParams>,
);
impl Schema for ListeriaStorageInventoryEndpointsGetItemsListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"isPending\":{\"type\":\"boolean\"},\"linkToExplorer\":{\"type\":\"string\"},\"transactionHash\":{\"type\":\"string\"},\"effects\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"statName\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"power\":{\"type\":\"string\"}},\"required\":[\"statName\",\"power\"]}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"equipmentItemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"}},\"required\":[\"equipmentItemId\",\"bindingId\",\"linkToExplorer\",\"effects\",\"isPending\",\"level\",\"maxLevel\",\"toNextLevelScrolls\",\"status\"]}}")
    }
}
impl Agent for ListeriaStorageInventoryEndpointsGetItemsListReturns {
    fn topic() -> &'static str {
        "listeria-storage_inventoryEndpoints_getItemsList"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getItemsList"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
}
