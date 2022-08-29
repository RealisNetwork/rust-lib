// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalDenyParams {
    #[serde(rename = "attemptId")]
    pub attempt_id: f64,
}
impl Schema for WithdrawApprovalDenyParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"attemptId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"attemptId\"]}")
    }
}
impl Agent for WithdrawApprovalDenyParams {
    fn topic() -> &'static str {
        "withdraw_approval_deny"
    }
    fn method() -> &'static str {
        "approval_deny"
    }
    fn agent() -> &'static str {
        "withdraw"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawApprovalDenyReturns(pub bool);
impl Schema for WithdrawApprovalDenyReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for WithdrawApprovalDenyReturns {
    fn topic() -> &'static str {
        "withdraw_approval_deny"
    }
    fn method() -> &'static str {
        "approval_deny"
    }
    fn agent() -> &'static str {
        "withdraw"
    }
}