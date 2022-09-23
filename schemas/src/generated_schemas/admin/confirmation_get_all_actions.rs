// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsParams {
    #[serde(rename = "page")]
    pub page: f64,
    #[serde(rename = "perPage")]
    pub per_page: f64,
}
impl Schema for AdminConfirmationGetAllActionsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}")
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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParamsParamsParams {}
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
    #[serde(rename = "infoMethod")]
    pub info_method: AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParams,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
    #[serde(rename = "createdAt")]
    pub created_at: String,
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
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"infoMethod\":{\"type\":\"object\",\"properties\":{\"params\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"agent\":{\"type\":\"string\"},\"method\":{\"type\":\"string\"}},\"required\":[\"agent\",\"method\",\"params\"]},\"userId\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"nickname\":{\"type\":\"string\"},\"isSuccess\":{\"type\":\"boolean\"},\"createdAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"isSuccess\",\"infoMethod\",\"nickname\",\"createdAt\",\"updatedAt\"]}},\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"data\",\"totalCount\"]}")
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
}
