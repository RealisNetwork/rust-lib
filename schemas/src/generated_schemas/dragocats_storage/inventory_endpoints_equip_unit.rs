// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsEquipUnitParams {
    #[serde(rename = "unitId")]
    pub unit_id: i32,
    #[serde(rename = "slotId")]
    pub slot_id: i32,
}
impl Schema for DragocatsStorageInventoryEndpointsEquipUnitParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsEquipUnitReturns {
    #[serde(rename = "slotId")]
    pub slot_id: i32,
    #[serde(rename = "unitId")]
    pub unit_id: i32,
}
