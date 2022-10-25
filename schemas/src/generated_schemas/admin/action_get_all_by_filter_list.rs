// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListParams {
    #[serde(rename = "lastDate")]
    pub last_date: Option<String>,
    #[serde(rename = "firstDate")]
    pub first_date: Option<String>,
    #[serde(rename = "agent")]
    pub agent: Option<String>,
    #[serde(rename = "page")]
    pub page: Option<f64>,
    #[serde(rename = "isCancelable")]
    pub is_cancelable: Option<bool>,
    #[serde(rename = "method")]
    pub method: Option<String>,
    #[serde(rename = "perPage")]
    pub per_page: Option<f64>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "nickname")]
    pub nickname: Option<String>,
}
impl Schema for AdminActionGetAllByFilterListParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"lastDate\":{\"type\":\"string\"},\"firstDate\":{\"type\":\"string\"},\"agent\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"isCancelable\":{\"type\":\"boolean\"},\"method\":{\"type\":\"string\"},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"}}}") . unwrap ()
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
pub struct AdminActionGetAllByFilterListReturnsDataParamsParamsParamsParams(pub Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminActionGetAllByFilterListReturnsDataParamsParams {
    #[serde(rename = "nickname", deserialize_with = "deserialize_to_string")]
    pub nickname: String,
    #[serde(rename = "method", deserialize_with = "deserialize_to_string")]
    pub method: String,
    #[serde(rename = "actionId", deserialize_with = "deserialize_to_string")]
    pub action_id: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "params")]
    pub params: AdminActionGetAllByFilterListReturnsDataParamsParamsParamsParams,
    #[serde(rename = "agent", deserialize_with = "deserialize_to_string")]
    pub agent: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_to_string")]
    pub created_at: String,
    #[serde(rename = "isCancelable")]
    pub is_cancelable: bool,
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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"method\":{\"type\":\"string\"},\"actionId\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"params\":{\"type\":\"object\",\"properties\":{}},\"agent\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"string\"},\"isCancelable\":{\"type\":\"boolean\"}},\"required\":[\"id\",\"actionId\",\"nickname\",\"userId\",\"method\",\"isCancelable\",\"params\",\"agent\",\"createdAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"data\",\"totalCount\"]}")
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
