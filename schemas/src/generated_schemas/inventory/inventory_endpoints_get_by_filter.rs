// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByFilterParamsOrderByParams {
    #[serde(rename = "desc")]
    pub desc: bool,
    #[serde(rename = "column")]
    pub column: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByFilterParamsFiltersParamsParams {
    #[serde(rename = "column")]
    pub column: String,
    #[serde(rename = "value")]
    pub value: (),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByFilterParams {
    #[serde(rename = "perPage")]
    pub per_page: i32,
    #[serde(rename = "page")]
    pub page: i32,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "orderBy")]
    pub order_by: Option<InventoryInventoryEndpointsGetByFilterParamsOrderByParams>,
    #[serde(rename = "filters")]
    pub filters: Option<Vec<InventoryInventoryEndpointsGetByFilterParamsFiltersParamsParams>>,
}
impl Schema for InventoryInventoryEndpointsGetByFilterParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":1,\"maximum\":100,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"page\":{\"type\":\"integer\",\"minimum\":1,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"category\":{\"type\":\"string\"},\"orderBy\":{\"type\":\"object\",\"properties\":{\"desc\":{\"type\":\"boolean\"},\"column\":{\"type\":\"string\"}},\"required\":[\"column\",\"desc\"]},\"filters\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"column\":{\"type\":\"string\"},\"value\":{}},\"required\":[\"column\",\"value\"]}}},\"required\":[\"category\",\"page\",\"perPage\"]}") . unwrap ()
    }
}
impl Agent for InventoryInventoryEndpointsGetByFilterParams {
    fn topic() -> &'static str {
        "inventory_inventoryEndpoints_getByFilter"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getByFilter"
    }
    fn agent() -> &'static str {
        "inventory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByFilterReturnsItemsParamsParamsAdditionalParamsParams(
    Value,
);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByFilterReturnsItemsParamsParams {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "additionalParams")]
    pub additional_params:
        InventoryInventoryEndpointsGetByFilterReturnsItemsParamsParamsAdditionalParamsParams,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "createdAt")]
    pub created_at: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: i32,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "subType")]
    pub sub_type: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "productId")]
    pub product_id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryInventoryEndpointsGetByFilterReturns {
    #[serde(rename = "totalCount")]
    pub total_count: i32,
    #[serde(rename = "items")]
    pub items: Vec<InventoryInventoryEndpointsGetByFilterReturnsItemsParamsParams>,
}
impl Schema for InventoryInventoryEndpointsGetByFilterReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalCount\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"items\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"nickname\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"additionalParams\":{\"type\":\"object\",\"properties\":{}},\"category\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}},\"description\":{\"type\":\"string\"},\"image\":{\"type\":\"string\"},\"type\":{\"type\":\"string\"},\"subType\":{\"type\":\"string\"},\"status\":{\"type\":\"string\"},\"productId\":{\"type\":\"integer\",\"minimum\":-2147483648,\"maximum\":2147483647,\"additionalAttributes\":{\"numberType\":\"Int\"}}},\"required\":[\"id\",\"productId\",\"userId\",\"nickname\",\"type\",\"subType\",\"description\",\"image\",\"additionalParams\",\"status\",\"category\",\"createdAt\",\"updatedAt\"]}}},\"required\":[\"items\",\"totalCount\"]}")
    }
}
impl Agent for InventoryInventoryEndpointsGetByFilterReturns {
    fn topic() -> &'static str {
        "inventory_inventoryEndpoints_getByFilter"
    }
    fn method() -> &'static str {
        "inventoryEndpoints_getByFilter"
    }
    fn agent() -> &'static str {
        "inventory"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Protected
    }
}
