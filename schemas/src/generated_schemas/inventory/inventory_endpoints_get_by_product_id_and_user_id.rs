// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByProductIdAndUserIdParams {
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for InventoryInventoryEndpointsGetByProductIdAndUserIdParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"productId\",\"userId\"]}") . unwrap ()
    }
}
impl Agent for InventoryInventoryEndpointsGetByProductIdAndUserIdParams {
    fn topic() -> &'static str {
        "inventory_inventoryEndpoints_getByProductIdAndUserId"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getByProductIdAndUserId"
    }
    fn agent() -> &'static str {
        "inventory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByProductIdAndUserIdReturnsAdditionalParamsParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByProductIdAndUserIdReturns {
    #[serde(rename = "additionalParams")]
    pub additional_params:
        InventoryInventoryEndpointsGetByProductIdAndUserIdReturnsAdditionalParamsParams,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "productId")]
    pub product_id: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
}
impl Schema for InventoryInventoryEndpointsGetByProductIdAndUserIdReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"status\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"type\":{\"type\":\"string\"},\"category\":{\"type\":\"string\"},\"subType\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"userId\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"description\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"nickname\",\"type\",\"subType\",\"description\",\"image\",\"additionalParams\",\"status\",\"category\",\"createdAt\",\"updatedAt\"]}")
    }
}
impl Agent for InventoryInventoryEndpointsGetByProductIdAndUserIdReturns {
    fn topic() -> &'static str {
        "inventory_inventoryEndpoints_getByProductIdAndUserId"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getByProductIdAndUserId"
    }
    fn agent() -> &'static str {
        "inventory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
