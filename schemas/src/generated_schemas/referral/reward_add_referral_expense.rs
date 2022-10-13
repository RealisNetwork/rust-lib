// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRewardAddReferralExpenseParams {
    #[serde(rename = "referralId")]
    pub referral_id: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
impl Schema for ReferralRewardAddReferralExpenseParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"referralId\":{\"type\":\"string\"},\"txId\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"referralId\",\"amount\",\"txId\"]}") . unwrap ()
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralRewardAddReferralExpenseReturns(pub bool);
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
    fn access_level() -> AccessLevel {
        AccessLevel::Internal
    }
}
