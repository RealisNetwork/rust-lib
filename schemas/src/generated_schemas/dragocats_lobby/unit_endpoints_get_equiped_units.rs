// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsGetEquipedUnitsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for DragocatsLobbyUnitEndpointsGetEquipedUnitsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\"]}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsGetEquipedUnitsReturnsParams {
    #[serde(rename = "unitId")]
    pub unit_id: i64,
    #[serde(rename = "slotId")]
    pub slot_id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsLobbyUnitEndpointsGetEquipedUnitsReturns(
    Vec<DragocatsLobbyUnitEndpointsGetEquipedUnitsReturnsParams>,
);
impl Schema for DragocatsLobbyUnitEndpointsGetEquipedUnitsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"unitId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"slotId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"unitId\",\"slotId\"]}}")
    }
}