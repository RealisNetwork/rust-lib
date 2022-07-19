// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRewardAddReferralExpenseParams {
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "referralId")]
    pub referral_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for ReferralRewardAddReferralExpenseParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"txId\":{\"type\":\"string\"},\"referralId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"referralId\",\"amount\",\"txId\"]}")
    }
}
impl Agent for ReferralRewardAddReferralExpenseParams {
    fn topic() -> &'static str {
        "referral_reward_addReferralExpense"
    }
    fn method() -> &'static str {
        "reward_addReferralExpense"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRewardAddReferralExpenseReturns(bool);
impl Schema for ReferralRewardAddReferralExpenseReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for ReferralRewardAddReferralExpenseReturns {
    fn topic() -> &'static str {
        "referral_reward_addReferralExpense"
    }
    fn method() -> &'static str {
        "reward_addReferralExpense"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
