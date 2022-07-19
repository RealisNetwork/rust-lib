// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for LobbyEquipmentEndpointsAddedItemNotificationParams {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LobbyEquipmentEndpointsAddedItemNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct LobbyEquipmentEndpointsAddedItemNotificationParams;
impl Schema for LobbyEquipmentEndpointsAddedItemNotificationParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsAddedItemNotificationReturnsEffectsParamsParams {
    #[serde(rename = "statName")]
    pub stat_name: i8,
    #[serde(rename = "power")]
    pub power: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsAddedItemNotificationReturns {
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "effects")]
    pub effects: Vec<LobbyEquipmentEndpointsAddedItemNotificationReturnsEffectsParamsParams>,
    #[serde(rename = "equipmentItemId")]
    pub equipment_item_id: i32,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
}
impl Schema for LobbyEquipmentEndpointsAddedItemNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"transactionHash\":{\"type\":\"string\"},\"status\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"toNextLevelScrolls\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"blockId\":{\"type\":\"string\"},\"effects\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"statName\":{\"type\":\"integer\",\"minimum\":-128,\"maximum\":127,\"additionalAttributes\":{\"numberType\":\"Byte\"}},\"power\":{\"type\":\"string\"}},\"required\":[\"statName\",\"power\"]}},\"equipmentItemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"linkToExplorer\":{\"type\":\"string\"},\"isPending\":{\"type\":\"boolean\"},\"level\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}},\"maxLevel\":{\"type\":\"integer\",\"minimum\":-32768,\"maximum\":32767,\"additionalAttributes\":{\"numberType\":\"Short\"}}},\"required\":[\"equipmentItemId\",\"bindingId\",\"linkToExplorer\",\"effects\",\"isPending\",\"level\",\"maxLevel\",\"toNextLevelScrolls\",\"status\"]}")
    }
}
