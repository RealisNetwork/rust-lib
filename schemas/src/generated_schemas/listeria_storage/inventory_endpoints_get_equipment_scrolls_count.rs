// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountParams;
impl Schema for ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountParams {
    fn topic() -> &'static str {
        "listeria-storage_inventoryEndpoints_getEquipmentScrollsCount"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getEquipmentScrollsCount"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountReturns(pub i32);
impl Schema for ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}")
    }
}
impl Agent for ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountReturns {
    fn topic() -> &'static str {
        "listeria-storage_inventoryEndpoints_getEquipmentScrollsCount"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getEquipmentScrollsCount"
    }
    fn agent() -> &'static str {
        "listeria-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
