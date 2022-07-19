// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsUnEquipUnitParams {
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
impl Schema for DragocatsLobbyUnitEndpointsUnEquipUnitParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsUnEquipUnitReturns {
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
impl Schema for DragocatsLobbyUnitEndpointsUnEquipUnitReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"unitId\"]}")
    }
}