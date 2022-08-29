// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsEquipUnitParams {
    #[serde(rename = "unitId")]
    pub unit_id: i32,
    #[serde(rename = "slotId")]
    pub slot_id: i32,
}
impl Schema for DragocatsLobbyUnitEndpointsEquipUnitParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"slotId\":{\"type\":\"integer\",\"minimum\":0,\"maximum\":2,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitId\",\"slotId\"]}")
    }
}
impl Agent for DragocatsLobbyUnitEndpointsEquipUnitParams {
    fn topic() -> &'static str {
        "dragocats-lobby_unitEndpoints_equipUnit"
    }
    fn method() -> &'static str {
        "unitEndpoints_equipUnit"
    }
    fn agent() -> &'static str {
        "dragocats-lobby"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsEquipUnitReturns {
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
impl Schema for DragocatsLobbyUnitEndpointsEquipUnitReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitId\",\"slotId\"]}")
    }
}
impl Agent for DragocatsLobbyUnitEndpointsEquipUnitReturns {
    fn topic() -> &'static str {
        "dragocats-lobby_unitEndpoints_equipUnit"
    }
    fn method() -> &'static str {
        "unitEndpoints_equipUnit"
    }
    fn agent() -> &'static str {
        "dragocats-lobby"
    }
}
