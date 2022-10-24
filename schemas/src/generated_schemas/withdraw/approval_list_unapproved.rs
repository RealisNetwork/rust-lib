// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedParams {
    #[serde(rename = "perPage")]
    pub per_page: Option<f64>,
    #[serde(rename = "page")]
    pub page: Option<f64>,
}
impl Schema for WithdrawApprovalListUnapprovedParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"perPage\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"page\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}}}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedReturnsParamsApproveDataParams {
    #[serde(rename = "whoConsidered")]
    pub who_considered: Option<String>,
    #[serde(rename = "approveReason")]
    pub approve_reason: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedReturnsParams {
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    #[serde(
        rename = "recipientAddress",
        deserialize_with = "deserialize_to_string"
    )]
    pub recipient_address: String,
    #[serde(rename = "approveData")]
    pub approve_data: WithdrawApprovalListUnapprovedReturnsParamsApproveDataParams,
    #[serde(rename = "id")]
    pub id: f64,
    #[serde(rename = "fee", deserialize_with = "deserialize_to_string")]
    pub fee: String,
    #[serde(rename = "userId", deserialize_with = "deserialize_to_string")]
    pub user_id: String,
    #[serde(rename = "amount", deserialize_with = "deserialize_to_string")]
    pub amount: String,
    #[serde(rename = "status", deserialize_with = "deserialize_to_string")]
    pub status: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalListUnapprovedReturns(
    pub Vec<WithdrawApprovalListUnapprovedReturnsParams>,
);
impl Schema for WithdrawApprovalListUnapprovedReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"createdAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"recipientAddress\":{\"type\":\"string\"},\"approveData\":{\"type\":\"object\",\"properties\":{\"whoConsidered\":{\"type\":\"string\"},\"approveReason\":{\"type\":\"string\"}}},\"id\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"fee\":{\"type\":\"string\"},\"userId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"status\":{\"type\":\"string\",\"pattern\":\"^(raw)|(success)$\"},\"updatedAt\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"id\",\"userId\",\"amount\",\"fee\",\"recipientAddress\",\"status\",\"approveData\",\"createdAt\",\"updatedAt\"]}}")
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
    fn access_level() -> AccessLevel {
        AccessLevel::Private
    }
}
