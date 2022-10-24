// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByProductIdParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
}
impl Schema for InventoryInventoryEndpointsGetByProductIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"productId\"]}") . unwrap ()
    }
}
impl Agent for InventoryInventoryEndpointsGetByProductIdParams {
    fn topic() -> &'static str {
        "inventory_inventoryEndpoints_getByProductId"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getByProductId"
    }
    fn agent() -> &'static str {
        "inventory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByProductIdReturnsAdditionalParamsParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByProductIdReturns {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "additionalParams")]
    pub additional_params: InventoryInventoryEndpointsGetByProductIdReturnsAdditionalParamsParams,
}
impl Schema for InventoryInventoryEndpointsGetByProductIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"},\"subType\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"description\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"nickname\":{\"type\":\"string\"},\"category\":{\"type\":\"string\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{}}},\"required\":[\"id\",\"productId\",\"userId\",\"nickname\",\"type\",\"subType\",\"description\",\"image\",\"additionalParams\",\"status\",\"category\",\"createdAt\",\"updatedAt\"]}")
    }
}
impl Agent for InventoryInventoryEndpointsGetByProductIdReturns {
    fn topic() -> &'static str {
        "inventory_inventoryEndpoints_getByProductId"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getByProductId"
    }
    fn agent() -> &'static str {
        "inventory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
