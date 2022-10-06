// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::generated_schemas::prelude::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralAddReferralParams {
    #[serde(rename = "referrerId")]
    pub referrer_id: String,
    #[serde(rename = "referralId")]
    pub referral_id: String,
}
impl Schema for ReferralReferralAddReferralParams {
    fn schema() -> Value {
        serde_json :: from_str ("{\"type\":\"object\",\"properties\":{\"referrerId\":{\"type\":\"string\"},\"referralId\":{\"type\":\"string\"}},\"required\":[\"referralId\",\"referrerId\"]}") . unwrap ()
    }
}
impl Agent for ReferralReferralAddReferralParams {
    fn topic() -> &'static str {
        "referral_referral_addReferral"
    }
    fn method() -> &'static str {
        "referral_addReferral"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralAddReferralReturns(pub bool);
impl Schema for ReferralReferralAddReferralReturns {
    fn schema() -> Value {
        serde_json::json!("{\"type\":\"boolean\"}")
    }
}
impl Agent for ReferralReferralAddReferralReturns {
    fn topic() -> &'static str {
        "referral_referral_addReferral"
    }
    fn method() -> &'static str {
        "referral_addReferral"
    }
    fn agent() -> &'static str {
        "referral"
    }
}
