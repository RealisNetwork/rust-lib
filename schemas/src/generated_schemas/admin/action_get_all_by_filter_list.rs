// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListParams {
    #[serde(rename = "isCancelable")]
    pub is_cancelable: Option<bool>,
    #[serde(rename = "lastDate")]
    pub last_date: Option<String>,
    #[serde(rename = "agent")]
    pub agent: Option<String>,
    #[serde(rename = "page")]
    pub page: Option<f64>,
    #[serde(rename = "nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "perPage")]
    pub per_page: Option<f64>,
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(rename = "firstDate")]
    pub first_date: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}
impl Schema for AdminActionGetAllByFilterListParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"isCancelable\":{\"type\":\"boolean\"},\"lastDate\":{\"type\":\"string\"},\"agent\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"nickname\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"method\":{\"type\":\"string\"},\"firstDate\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":null}")
    }
}
impl Agent for AdminActionGetAllByFilterListParams {
    fn topic() -> &'static str {
        "admin_action_getAllByFilterList"
    }
    fn method() -> &'static str {
        "action_getAllByFilterList"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturnsDataParamsParamsParamsParams {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturnsDataParamsParams {
    #[serde(rename = "agent")]
    pub agent: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isCancelable")]
    pub is_cancelable: bool,
    #[serde(rename = "params")]
    pub params: AdminActionGetAllByFilterListReturnsDataParamsParamsParamsParams,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "userId")]
    pub user_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturns {
    #[serde(rename = "data")]
    pub data: Vec<AdminActionGetAllByFilterListReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}
impl Schema for AdminActionGetAllByFilterListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"agent\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"isCancelable\":{\"type\":\"boolean\"},\"params\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"method\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"actionId\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"}},\"required\":[\"id\",\"actionId\",\"nickname\",\"userId\",\"method\",\"isCancelable\",\"params\",\"agent\",\"createdAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"data\",\"totalCount\"]}")
    }
}
impl Agent for AdminActionGetAllByFilterListReturns {
    fn topic() -> &'static str {
        "admin_action_getAllByFilterList"
    }
    fn method() -> &'static str {
        "action_getAllByFilterList"
    }
    fn agent() -> &'static str {
        "admin"
    }
}
