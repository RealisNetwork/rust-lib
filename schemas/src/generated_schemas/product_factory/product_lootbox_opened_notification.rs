// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
impl<'de> Deserialize<'de> for ProductFactoryProductLootboxOpenedNotificationParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(ProductFactoryProductLootboxOpenedNotificationParams)
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct ProductFactoryProductLootboxOpenedNotificationParams;
impl Schema for ProductFactoryProductLootboxOpenedNotificationParams {
    fn schema() -> Value {
        todo!()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductLootboxOpenedNotificationReturnsRewardsParamsParams {
    #[serde(rename = "type")]
    pub r#type: i32,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "itemId")]
    pub item_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductFactoryProductLootboxOpenedNotificationReturns {
    #[serde(rename = "rewards")]
    pub rewards: Vec<ProductFactoryProductLootboxOpenedNotificationReturnsRewardsParamsParams>,
    #[serde(rename = "bindingId")]
    pub binding_id: i32,
}
