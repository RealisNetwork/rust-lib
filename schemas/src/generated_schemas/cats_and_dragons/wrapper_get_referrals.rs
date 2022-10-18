// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "userId")]
    pub user_id: String,
}
impl Schema for CatsAndDragonsWrapperGetReferralsParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"userId\":{\"type\":\"string\"}},\"required\":[\"userId\",\"appId\"]}") . unwrap ()
    }
}
impl Agent for CatsAndDragonsWrapperGetReferralsParams {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getReferrals"
    }
    fn method() -> &'static str {
        "wrapper_getReferrals"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsReturnsReferralTransactionsParamsParams {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "amount")]
    pub amount: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsReturnsReferralsParamsParams {
    #[serde(rename = "nickname")]
    pub nickname: String,
    #[serde(rename = "appId")]
    pub app_id: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsReturns {
    #[serde(rename = "referralTransactions")]
    pub referral_transactions:
        Vec<CatsAndDragonsWrapperGetReferralsReturnsReferralTransactionsParamsParams>,
    #[serde(rename = "referrals")]
    pub referrals: Vec<CatsAndDragonsWrapperGetReferralsReturnsReferralsParamsParams>,
}
impl Schema for CatsAndDragonsWrapperGetReferralsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referralTransactions\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"date\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"}},\"required\":[\"date\",\"nickname\",\"amount\"]}},\"referrals\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"nickname\":{\"type\":\"string\"},\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}}},\"required\":[\"nickname\",\"appId\"]}}},\"required\":[\"referrals\",\"referralTransactions\"]}")
    }
}
impl Agent for CatsAndDragonsWrapperGetReferralsReturns {
    fn topic() -> &'static str {
        "catsAndDragons_wrapper_getReferrals"
    }
    fn method() -> &'static str {
        "wrapper_getReferrals"
    }
    fn agent() -> &'static str {
        "catsAndDragons"
    }
    fn access_level() -> AccessLevel {
        AccessLevel::Public
    }
}
