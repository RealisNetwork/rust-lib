// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsEquipUnitParams {
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
impl Schema for DragocatsStorageInventoryEndpointsEquipUnitParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"slotId\",\"unitId\"]}") . unwrap ()
    }
}
impl Agent for DragocatsStorageInventoryEndpointsEquipUnitParams {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_equipUnit"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_equipUnit"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsEquipUnitReturns {
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
impl Schema for DragocatsStorageInventoryEndpointsEquipUnitReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"slotId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"unitId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"slotId\",\"unitId\"]}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsEquipUnitReturns {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_equipUnit"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_equipUnit"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
