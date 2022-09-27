// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsParams {
    #[serde(rename = "perPage")]
    pub per_page: f64,
    #[serde(rename = "page")]
    pub page: f64,
}
impl Schema for AdminConfirmationGetAllActionsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"page\",\"perPage\"]}")
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
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "params")]
    pub params: AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParamsParamsParams,
    #[serde(rename = "agent")]
    pub agent: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturnsDataParamsParams {
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "isSuccess")]
    pub is_success: bool,
    #[serde(rename = "infoMethod")]
    pub info_method: AdminConfirmationGetAllActionsReturnsDataParamsParamsInfoMethodParams,
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfirmationGetAllActionsReturns {
    #[serde(rename = "totalCount")]
    pub total_count: f64,
    #[serde(rename = "data")]
    pub data: Vec<AdminConfirmationGetAllActionsReturnsDataParamsParams>,
}
impl Schema for AdminConfirmationGetAllActionsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"totalCount\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"data\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"isSuccess\":{\"type\":\"boolean\"},\"infoMethod\":{\"type\":\"object\",\"properties\":{\"method\":{\"type\":\"string\"},\"params\":{\"type\":\"object\",\"properties\":{},\"required\":null},\"agent\":{\"type\":\"string\"}},\"required\":[\"agent\",\"method\",\"params\"]},\"nickname\":{\"type\":\"string\"},\"createdAt\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"isSuccess\",\"infoMethod\",\"nickname\",\"createdAt\",\"updatedAt\"]}}},\"required\":[\"data\",\"totalCount\"]}")
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
