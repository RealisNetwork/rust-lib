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
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsReturnsReferralsParamsParams {
    #[serde(rename = "appId")]
    pub app_id: f64,
    #[serde(rename = "nickname")]
    pub nickname: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsReturnsReferralTransactionsParamsParams {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "nickname")]
    pub nickname: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatsAndDragonsWrapperGetReferralsReturns {
    #[serde(rename = "referrals")]
    pub referrals: Vec<CatsAndDragonsWrapperGetReferralsReturnsReferralsParamsParams>,
    #[serde(rename = "referralTransactions")]
    pub referral_transactions:
        Vec<CatsAndDragonsWrapperGetReferralsReturnsReferralTransactionsParamsParams>,
}
impl Schema for CatsAndDragonsWrapperGetReferralsReturns {
    fn schema() -> Value {
        serde_json :: json ! ("{\"type\":\"object\",\"properties\":{\"referrals\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"appId\":{\"type\":\"integer\",\"minimum\":-9223372036854775808,\"maximum\":9223372036854775807,\"additionalAttributes\":{\"numberType\":\"Number\"}},\"nickname\":{\"type\":\"string\"}},\"required\":[\"nickname\",\"appId\"]}},\"referralTransactions\":{\"type\":\"array\",\"items\":{\"type\":\"object\",\"properties\":{\"date\":{\"type\":\"string\"},\"amount\":{\"type\":\"string\"},\"nickname\":{\"type\":\"string\"}},\"required\":[\"date\",\"nickname\",\"amount\"]}}},\"required\":[\"referrals\",\"referralTransactions\"]}")
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
}
