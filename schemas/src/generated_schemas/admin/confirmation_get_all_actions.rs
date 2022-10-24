// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsParams {
    #[serde(rename = "perPage")]
    pub per_page: f64,
    #[serde(rename = "tab")]
    pub tab: String,
    #[serde(rename = "page")]
    pub page: f64,
}
impl Schema for AdminConfirmationGetAllActionsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"tab\":{\"type\":\"string\"},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\",\"tab\"]}") . unwrap ()
    }
}
impl Agent for AdminConfirmationGetAllActionsParams {
    fn topic() -> &'static str {
        "admin_confirmation_getAllActions"
    }
    fn method() -> &'static str {
        "confirmation_getAllActions"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParamsParamsParams(Value);
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParams {
    #[serde(rename = "params")]
    pub params: AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParamsParamsParams,
    #[serde(rename = "agent")]
    pub agent: String,
    #[serde(rename = "method")]
    pub method: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturnsDataParamsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "infoMethod")]
    pub info_method: AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParams,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "tab")]
    pub tab: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturns {
    #[serde(rename = "data")]
    pub data: Vec<AdminConfirmationGetAllActionsReturnsDataParamsParams>,
    #[serde(rename = "totalCount")]
    pub total_count: f64,
}
impl Schema for AdminConfirmationGetAllActionsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"},\"infoMethod\":{\"type\":\"object\",\"properties\":{\"params\":{\"type\":\"object\",\"properties\":{}},\"agent\":{\"type\":\"string\"},\"method\":{\"type\":\"string\"}},\"required\":[\"agent\",\"method\",\"params\"]},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"updatedAt\":{\"type\":\"string\"},\"isSuccess\":{\"type\":\"boolean\"},\"createdAt\":{\"type\":\"string\"},\"tab\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"isSuccess\",\"infoMethod\",\"tab\",\"nickname\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"data\",\"totalCount\"]}")
    }
}
impl Agent for AdminConfirmationGetAllActionsReturns {
    fn topic() -> &'static str {
        "admin_confirmation_getAllActions"
    }
    fn method() -> &'static str {
        "confirmation_getAllActions"
    }
    fn agent() -> &'static str {
        "admin"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
