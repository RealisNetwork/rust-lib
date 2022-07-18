// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
pub type ListeriaStorageInventoryEndpointsGetItemsListParams = ();
#[derive(Debug, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetItemsListReturnsParamsEffectsParamsParams {
    #[serde(rename = "statName")]
    pub stat_name: i8,
    #[serde(rename = "power")]
    pub power: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ListeriaStorageInventoryEndpointsGetItemsListReturnsParams {
    #[serde(rename = "blockId")]
    pub block_id: Option<String>,
    #[serde(rename = "status")]
    pub status: i32,
    #[serde(rename = "isPending")]
    pub is_pending: bool,
    #[serde(rename = "toNextLevelScrolls")]
    pub to_next_level_scrolls: i32,
    #[serde(rename = "equipmentItemId")]
    pub equipment_item_id: i32,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
    #[serde(rename = "linkToExplorer")]
    pub link_to_explorer: String,
    #[serde(rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "effects")]
    pub effects: Vec<ListeriaStorageInventoryEndpointsGetItemsListReturnsParamsEffectsParamsParams>,
    #[serde(rename = "level")]
    pub level: i16,
    #[serde(rename = "maxLevel")]
    pub max_level: i16,
}
pub type ListeriaStorageInventoryEndpointsGetItemsListReturns =
    Vec<ListeriaStorageInventoryEndpointsGetItemsListReturnsParams>;
