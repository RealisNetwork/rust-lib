// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
pub type ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationParams = ();
#[derive(Debug, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationReturns {
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}
