// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsEquipItemParams {
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}
impl Schema for LobbyEquipmentEndpointsEquipItemParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"itemId\",\"slotId\",\"heroId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyEquipmentEndpointsEquipItemReturns {
    #[serde(rename = "heroId")]
    pub hero_id: i32,
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}
impl Schema for LobbyEquipmentEndpointsEquipItemReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"heroId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"itemId\",\"slotId\",\"heroId\"]}")
    }
}