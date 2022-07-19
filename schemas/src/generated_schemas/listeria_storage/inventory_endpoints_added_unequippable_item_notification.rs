// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
impl<'de> Deserialize<'de>
    for ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationParams
{
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationParams;
impl Schema for ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationParams {
    fn schema() -> Value {
        serde_json::json!("{}")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationReturns {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
}
impl Schema for ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"itemId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"bindingId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"itemId\",\"bindingId\",\"type\"]}")
    }
}