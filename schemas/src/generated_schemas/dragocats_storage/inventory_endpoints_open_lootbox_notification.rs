// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de> for DragocatsStorageInventoryEndpointsOpenLootboxNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_json::Value::deserialize(deserializer)?;
        Ok(DragocatsStorageInventoryEndpointsOpenLootboxNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct DragocatsStorageInventoryEndpointsOpenLootboxNotificationParams;
impl Schema for DragocatsStorageInventoryEndpointsOpenLootboxNotificationParams {
    fn schema() -> Value {
        serde_json::from_str("{}").unwrap()
    }
}
impl Agent for DragocatsStorageInventoryEndpointsOpenLootboxNotificationParams {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_openLootboxNotification"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_openLootboxNotification"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragocatsStorageInventoryEndpointsOpenLootboxNotificationReturns(pub Value);
impl Schema for DragocatsStorageInventoryEndpointsOpenLootboxNotificationReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"object\",\"properties\":{}}")
    }
}
impl Agent for DragocatsStorageInventoryEndpointsOpenLootboxNotificationReturns {
    fn topic() -> &'static str {
        "dragocats-storage_inventoryEndpoints_openLootboxNotification"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_openLootboxNotification"
    }
    fn agent() -> &'static str {
        "dragocats-storage"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
