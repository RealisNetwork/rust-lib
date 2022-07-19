// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsParams {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
impl Schema for ReferralReferralGetReferralsParams {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"userId\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"userId\",\"appId\"]}")
    }
}
impl Agent for ReferralReferralGetReferralsParams {
    fn topic() -> &'static str {
        "referral_referral_getReferrals"
    }
    fn method() -> &'static str {
        "referral_getReferrals"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsReturnsReferralsParamsParams {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "appId")]
    pub app_id: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsReturnsReferralTransactionsParamsParams {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralGetReferralsReturns {
    #[serde(rename = "referrals")]
    pub referrals: Vec<ReferralReferralGetReferralsReturnsReferralsParamsParams>,
    #[serde(rename = "referralTransactions")]
    pub referral_transactions:
        Vec<ReferralReferralGetReferralsReturnsReferralTransactionsParamsParams>,
}
impl Schema for ReferralReferralGetReferralsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referrals\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"nickname\",\"appId\"]}},\"referralTransactions\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"date\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"date\",\"nickname\",\"amount\"]}}},\"required\":[\"referrals\",\"referralTransactions\"]}")
    }
}
impl Agent for ReferralReferralGetReferralsReturns {
    fn topic() -> &'static str {
        "referral_referral_getReferrals"
    }
    fn method() -> &'static str {
        "referral_getReferrals"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
