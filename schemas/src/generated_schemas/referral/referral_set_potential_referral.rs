// This file are autogenerated on build, everything you write here will be replaced

#![allow(unknown_lints)]
#![allow(clippy::all)]
use crate::Schema;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralReferralSetPotentialReferralParams {
    #[serde(rename = "referralId")]
    pub referral_id: String,
    #[serde(rename = "referrerId")]
    pub referrer_id: String,
}
impl Schema for ReferralReferralSetPotentialReferralParams {
    fn schema() -> Value {
        todo!()
    }
}
pub type ReferralReferralSetPotentialReferralReturns = bool;
