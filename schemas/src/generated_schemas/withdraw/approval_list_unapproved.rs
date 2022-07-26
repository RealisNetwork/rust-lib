// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedParams {
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "page")]
    pub page: i64,
}
impl Schema for WithdrawApprovalListUnapprovedParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":null}")
    }
}
impl Agent for WithdrawApprovalListUnapprovedParams {
    fn topic() -> &'static str {
        "withdraw_approval_listUnapproved"
    }
    fn method() -> &'static str {
        "approval_listUnapproved"
    }
    fn agent() -> &'static str {
        "withdraw"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedReturnsParamsApproveDataParams {
    #[serde(rename = "whoConsidered")]
    pub who_considered: String,
    #[serde(rename = "approveReason")]
    pub approve_reason: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedReturnsParams {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "recipientAddress")]
    pub recipient_address: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "approveData")]
    pub approve_data: WithdrawApprovalListUnapprovedReturnsParamsApproveDataParams,
    #[serde(rename = "fee")]
    pub fee: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedReturns(
    pub Vec<WithdrawApprovalListUnapprovedReturnsParams>,
);
impl Schema for WithdrawApprovalListUnapprovedReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"status\":{\"type\":\"string\",\"pattern\":\"^(raw)|(success)$\"},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"},\"recipientAddress\":{\"type\":\"string\"},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"createdAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"amount\":{\"type\":\"string\"},\"approveData\":{\"type\":\"object\",\"properties\":{\"whoConsidered\":{\"type\":\"string\"},\"approveReason\":{\"type\":\"string\"}},\"required\":null},\"fee\":{\"type\":\"string\"}},\"required\":[\"id\",\"userId\",\"amount\",\"fee\",\"recipientAddress\",\"status\",\"approveData\",\"createdAt\",\"updatedAt\"]}}")
    }
}
impl Agent for WithdrawApprovalListUnapprovedReturns {
    fn topic() -> &'static str {
        "withdraw_approval_listUnapproved"
    }
    fn method() -> &'static str {
        "approval_listUnapproved"
    }
    fn agent() -> &'static str {
        "withdraw"
    }
}
