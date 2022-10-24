// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListParams {
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "agent")]
    pub agent: Option<String>,
    #[serde(rename = "perPage")]
    pub per_page: Option<f64>,
    #[serde(rename = "nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "lastDate")]
    pub last_date: Option<String>,
    #[serde(rename = "page")]
    pub page: Option<f64>,
    #[serde(rename = "firstDate")]
    pub first_date: Option<String>,
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(rename = "isCancelable")]
    pub is_cancelable: Option<bool>,
}
impl Schema for AdminActionGetAllByFilterListParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"agent\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"nickname\":{\"type\":\"string\"},\"lastDate\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"firstDate\":{\"type\":\"string\"},\"method\":{\"type\":\"string\"},\"isCancelable\":{\"type\":\"boolean\"}}}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturnsDataParamsParamsParamsParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturnsDataParamsParams {
    #[serde(rename = "params")]
    pub params: AdminActionGetAllByFilterListReturnsDataParamsParamsParamsParams,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isCancelable")]
    pub is_cancelable: bool,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "agent")]
    pub agent: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "actionId")]
    pub action_id: String,
    #[serde(rename = "id")]
    pub id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturns {
    #[serde(rename = "totalCount")]
    pub total_count: f64,
    #[serde(rename = "data")]
    pub data: Vec<AdminActionGetAllByFilterListReturnsDataParamsParams>,
}
impl Schema for AdminActionGetAllByFilterListReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"params\":{\"type\":\"object\",\"properties\":{}},\"createdAt\":{\"type\":\"string\"},\"isCancelable\":{\"type\":\"boolean\"},\"nickname\":{\"type\":\"string\"},\"method\":{\"type\":\"string\"},\"agent\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"actionId\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"actionId\",\"nickname\",\"userId\",\"method\",\"isCancelable\",\"params\",\"agent\",\"createdAt\"]}}},\"required\":[\"data\",\"totalCount\"]}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
