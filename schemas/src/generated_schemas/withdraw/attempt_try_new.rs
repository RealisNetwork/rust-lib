// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawAttemptTryNewParams {
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "receiverAddress")]
    pub receiver_address: String,
}
impl Schema for WithdrawAttemptTryNewParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"amount\":{\"type\":\"string\"},\"receiverAddress\":{\"type\":\"string\"}},\"required\":[\"receiverAddress\",\"amount\"]}")
    }
}
impl Agent for WithdrawAttemptTryNewParams {
    fn topic() -> &'static str {
        "withdraw_attempt_tryNew"
    }
    fn method() -> &'static str {
        "attempt_tryNew"
    }
    fn agent() -> &'static str {
        "withdraw"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawAttemptTryNewReturns(pub bool);
impl Schema for WithdrawAttemptTryNewReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for WithdrawAttemptTryNewReturns {
    fn topic() -> &'static str {
        "withdraw_attempt_tryNew"
    }
    fn method() -> &'static str {
        "attempt_tryNew"
    }
    fn agent() -> &'static str {
        "withdraw"
    }
}
